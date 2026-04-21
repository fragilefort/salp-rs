use serde::ser::{Serialize, SerializeMap, Serializer};

pub enum Query {
    Or(Vec<Query>),
    And(Vec<Query>),
    Organism(String),
    Keyword(String),
    MaxResolution(f32),
}

impl Serialize for Query {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Query::And(queries) => {
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("type", "group")?;
                map.serialize_entry("logical_operator", "and")?;
                map.serialize_entry("nodes", queries)?;
                map.end()
            }
            Query::Or(queries) => {
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("type", "group")?;
                map.serialize_entry("logical_operator", "or")?;
                map.serialize_entry("nodes", queries)?;
                map.end()
            }
            Query::Organism(org) => {
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("type", "terminal")?;
                map.serialize_entry("service", "text")?;
                map.serialize_entry(
                    "parameters",
                    &ParameterMap {
                        attribute: "rcsb_entity_source_organism.taxonomy_lineage.name",
                        operator: "exact_match",
                        value: ParameterValue::Str(org),
                    },
                )?;
                map.end()
            }

            Query::Keyword(kw) => {
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("type", "terminal")?;
                map.serialize_entry("service", "text")?;
                map.serialize_entry(
                    "parameters",
                    &ParameterMap {
                        attribute: "struct_keywords.pdbx_keywords",
                        operator: "contains_phrase",
                        value: ParameterValue::Str(kw),
                    },
                )?;
                map.end()
            }

            Query::MaxResolution(mr) => {
                let mut map = serializer.serialize_map(None)?;
                map.serialize_entry("type", "terminal")?;
                map.serialize_entry("service", "text")?;
                map.serialize_entry(
                    "parameters",
                    &ParameterMap {
                        attribute: "rcsb_entry_info.resolution_combined",
                        operator: "less_or_equal",
                        value: ParameterValue::Float(*mr),
                    },
                )?;
                map.end()
            }
        }
    }
}

enum ParameterValue<'a> {
    Str(&'a str),
    Float(f32),
}

impl<'a> Serialize for ParameterValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ParameterValue::Str(s) => serializer.serialize_str(s),
            ParameterValue::Float(f) => serializer.serialize_f32(*f),
        }
    }
}

struct ParameterMap<'a> {
    attribute: &'a str,
    operator: &'a str,
    value: ParameterValue<'a>,
}

impl<'a> Serialize for ParameterMap<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("attribute", self.attribute)?;
        map.serialize_entry("operator", self.operator)?;
        map.serialize_entry("value", &self.value)?;
        map.end()
    }
}

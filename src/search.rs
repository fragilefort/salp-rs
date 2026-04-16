pub enum Query {
    Or(Vec<Query>),
    And(Vec<Query>),
    Organism(String),
    Keyword(String),
    MaxResolution(f32),
}

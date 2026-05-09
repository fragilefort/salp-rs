use crate::search::Query;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct PyQuery {
    pub inner: Query,
}

#[pymethods]
impl PyQuery {
    #[staticmethod]
    pub fn origanism(org: String) -> PyQuery {
        PyQuery {
            inner: Query::Organism(org),
        }
    }

    #[staticmethod]
    pub fn keyword(kw: String) -> PyQuery {
        PyQuery {
            inner: Query::Keyword(kw),
        }
    }

    #[staticmethod]
    pub fn max_resolution(mr: f32) -> PyQuery {
        PyQuery {
            inner: Query::MaxResolution(mr),
        }
    }

    #[staticmethod]
    pub fn and_(queries: Vec<PyQuery>) -> PyQuery {
        PyQuery {
            inner: Query::And(queries.into_iter().map(|q| q.inner).collect()),
        }
    }

    #[staticmethod]
    pub fn or_(queries: Vec<PyQuery>) -> PyQuery {
        PyQuery {
            inner: Query::Or(queries.into_iter().map(|q| q.inner).collect()),
        }
    }
}

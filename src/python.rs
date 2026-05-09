use crate::process_response::parse_ids;
use crate::{rcsb_reqwest::SearchRequest, search::Query};
use pyo3::exceptions::PyRuntimeError;
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

#[pyfunction]
pub fn search(query: PyQuery, start: u32, rows: u32) -> PyResult<(u64, Vec<String>)> {
    let req = SearchRequest::new(&query.inner, start, rows);
    let response = req
        .post_request()
        .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("Request failed: {}", e)))?;

    match response {
        Some(resp) => {
            let parsed = parse_ids(&resp)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

            Ok(parsed)
        }
        None => Ok((0, vec![])),
    }
}

#[pymodule]
fn salp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyQuery>()?;
    m.add_function(wrap_pyfunction!(search, m)?)?;

    Ok(())
}

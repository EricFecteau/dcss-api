use crate::Webtile;
use crate::{BlockingError, Error};

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass]
/// This is a test
pub struct WebtilePy {
    webtile: Webtile,
}

pyo3::create_exception!(mymodule, APIErr, PyException);
pyo3::create_exception!(mymodule, BlockingErr, PyException);

#[pymethods]
impl WebtilePy {
    #[new]
    fn connect(url: &str, speed_ms: usize, version: &str) -> PyResult<Self> {
        let webtile = Webtile::connect(url, speed_ms, version);

        match webtile {
            Ok(t) => Ok(Self { webtile: t }),
            Err(e) => Err(PyErr::new::<APIErr, _>(e.to_string())),
        }
    }

    /// THIS IS TEST DOCSTRING
    fn read_until(&mut self, msg: &str, key: Option<&str>, value: Option<u64>) -> PyResult<()> {
        let result = self.webtile.read_until(msg, key, value);

        match result {
            Ok(t) => Ok(t),
            Err(e) => match e {
                Error::Blocking(BlockingError::Pickup) => {
                    Err(PyErr::new::<BlockingErr, _>("Pickup"))
                }
                _ => Err(PyErr::new::<APIErr, _>(e.to_string())),
            },
        }
    }

    fn write_key(&mut self, key: &str) -> PyResult<()> {
        self.webtile
            .write_key(key)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    /// Pydoc -- is it working2?
    fn write_json(&mut self, json: &str) -> PyResult<()> {
        self.webtile
            .write_json(serde_json::from_str(json).unwrap())
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    // fn login_with_credentials(&mut self, username: &str, password: &str) -> PyResult<()> {
    //     self.webtile
    //         .login_with_credentials(username, password)
    //         .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    // }

    fn get_message(&mut self) -> Option<String> {
        let value = self.webtile.received_messages.pop_front();
        value.map(|v| v.to_string())
    }
}

#[pymodule]
pub fn dcss_api(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<WebtilePy>()?;
    m.add("BlockingErr", py.get_type::<BlockingErr>())?;
    m.add("APIErr", py.get_type::<APIErr>())?;
    Ok(())
}

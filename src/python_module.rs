use crate::Webtile;

use anyhow::Result;
use pyo3::prelude::*;

#[pyclass]
pub struct WebtilePy {
    webtile: Webtile,
}

#[pymethods]
impl WebtilePy {
    #[new]
    fn connect(url: &str, speed_ms: usize, version: &str) -> Self {
        Self {
            webtile: Webtile::connect(url, speed_ms, version).expect(""),
        }
    }

    fn read_until(&mut self, msg: &str, key: Option<&str>, value: Option<u64>) -> Result<()> {
        self.webtile.read_until(msg, key, value)
    }

    fn write_key(&mut self, key: &str) {
        self.webtile.write_key(key).unwrap();
    }

    fn write_json(&mut self, json: &str) {
        self.webtile
            .write_json(serde_json::from_str(json).unwrap())
            .unwrap();
    }

    fn get_message(&mut self) -> Option<String> {
        let value = self.webtile.received_messages.pop_front();
        value.map(|v| v.to_string())
    }
}

#[pymodule]
pub fn dcss_api(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<WebtilePy>()?;
    Ok(())
}

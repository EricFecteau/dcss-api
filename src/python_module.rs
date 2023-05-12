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

    fn disconnect(&mut self) -> PyResult<()> {
        let result = self.webtile.disconnect();

        match result {
            Ok(t) => Ok(t),
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

    fn login_with_credentials(&mut self, username: &str, password: &str) -> PyResult<Vec<String>> {
        self.webtile
            .login_with_credentials(username, password)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn login_with_cookie(&mut self, cookie: &str) -> PyResult<Vec<String>> {
        self.webtile
            .login_with_cookie(cookie)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn request_cookie(&mut self) -> PyResult<String> {
        self.webtile
            .request_cookie()
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn get_message(&mut self) -> Option<String> {
        let value = self.webtile.received_messages.pop_front();
        value.map(|v| v.to_string())
    }

    fn set_rc_file(&mut self, game_id: &str, content: &str) -> PyResult<()> {
        self.webtile
            .set_rc_file(game_id, content)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn get_rc_file(&mut self, game_id: &str) -> PyResult<String> {
        self.webtile
            .get_rc_file(game_id)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn start_game(
        &mut self,
        game_id: &str,
        species: &str,
        background: &str,
        weapon: &str,
    ) -> PyResult<()> {
        self.webtile
            .start_game(game_id, species, background, weapon)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn start_game_seeded(
        &mut self,
        game_id: &str,
        seed: &str,
        pregenerate: bool,
        species: &str,
        background: &str,
        weapon: &str,
    ) -> PyResult<()> {
        self.webtile
            .start_game_seeded(game_id, seed, pregenerate, species, background, weapon)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn save_game(&mut self) -> PyResult<()> {
        self.webtile
            .save_game()
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn continue_game(&mut self, game_id: &str) -> PyResult<()> {
        self.webtile
            .continue_game(game_id)
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }

    fn quit_game(&mut self) -> PyResult<()> {
        self.webtile
            .quit_game()
            .map_err(|e| PyErr::new::<APIErr, _>(e.to_string()))
    }
}

#[pymodule]
pub fn dcss_api(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<WebtilePy>()?;
    m.add("BlockingErr", py.get_type::<BlockingErr>())?;
    m.add("APIErr", py.get_type::<APIErr>())?;
    Ok(())
}

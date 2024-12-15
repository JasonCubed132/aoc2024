use std::fmt::Display;

use anyhow::Result;

pub struct DisplayResult<'a, T> {
    inner: &'a Result<T>,
}

impl<'a, T> DisplayResult<'a, T> {
    pub fn new(inner: &'a Result<T>) -> Self {
        Self { inner }
    }
}

impl<'a, T: ToString> Display for DisplayResult<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .inner
            .as_ref()
            .and_then(|int| Ok(int.to_string()))
            .unwrap_or("None".to_string());
        write!(f, "{}", str)
    }
}

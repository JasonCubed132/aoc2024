use std::fmt::Display;

use anyhow::Result;

impl<T: ToString> Display for Result<T> {
    fn fmt(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .and_then(|int| Ok(int.to_string()))
            .unwrap_or("None".to_string());
        write!(f, "{}", str)
    }
}

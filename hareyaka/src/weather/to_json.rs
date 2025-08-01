use std::io::{Error, ErrorKind};
use crate::weather::Hareyaka;

impl Hareyaka {
    pub fn as_json(&self) -> Result<String, Error> {
        return serde_json::to_string(&self.data)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e));
    }
}
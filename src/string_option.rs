use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct StringOption<T>(pub Option<T>);

impl<T: fmt::Display> fmt::Display for StringOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(string) => write!(f, "{}", string),
            None => write!(f, ""),
        }
    }
}


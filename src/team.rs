use std::fmt;
use serde::{Serialize, Deserialize};
use cli_table::Table;

#[derive(Table, Serialize, Deserialize, Debug)]
pub struct Team {
    id: i32,
    name: String,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


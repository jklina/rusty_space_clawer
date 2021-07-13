use serde::{Serialize, Deserialize};
use cli_table::Table;
use std::fmt;

#[derive(Table, Serialize, Deserialize, Debug)]
pub struct Location {
    id: i32,
    name: String,
    x_coord: i32,
    y_coord: i32,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


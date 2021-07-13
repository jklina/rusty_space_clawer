use serde::Deserialize;
use cli_table::Table;
use std::fmt;

#[derive(Table, Deserialize, Debug)]
pub struct SpaceTime {
    nanos_since_epoch: i32,
    secs_since_epoch: i32,
}

impl fmt::Display for SpaceTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nanos_since_epoch)
    }
}


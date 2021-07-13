use crate::contract::Contract;
use crate::string_option::StringOption;
use crate::location::Location;
use serde::{Serialize, Deserialize};
use cli_table::Table;
use std::fmt;
use std::collections::HashMap;

#[derive(Table, Serialize, Deserialize, Debug)]
pub struct Ship {
    bearing: f32,
    #[table(skip)]
    cargo: HashMap<String, String>,
    #[serde(skip)]
    #[table(skip)]
    contracts: HashMap<String, Contract>,
    course: StringOption<String>,
    engine_heat: f32,
    fuel: f32,
    location: Location,
    max_cargo: i32,
    max_fuel: f32,
    next_waypoint_index: StringOption<String>,
    player_id: i32,
    #[table(skip)]
    position: (f32, f32),
    speed: f32,
    status: String,
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bearing: {}", self.bearing)
    }
}

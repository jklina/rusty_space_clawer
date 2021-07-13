use crate::space_time::SpaceTime;
use crate::location::Location;
use serde::Deserialize;
use cli_table::Table;

#[derive(Table, Deserialize, Debug)]
pub struct Contract {
    id: i32,
    destination: Location,
    cargo_type: String,
    current_cost: i32,
    origin: Location,
    expiration: SpaceTime,
    pay_rate: i32,
    starting_cost: i32,
    volume: i32,
}


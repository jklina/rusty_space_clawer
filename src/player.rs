use crate::team::Team;
use crate::ship::Ship;
use crate::space_time::SpaceTime;
use crate::contract::Contract;
use std::collections::HashMap;
use std::fmt;
use serde::Deserialize;
use cli_table::Table;

#[derive(Deserialize, Debug)]
pub struct Players {
    #[serde(flatten)]
    wrapped_players: HashMap<String, Player>,
}

impl Players {
    pub fn players(&self) -> Vec<&Player> {
        return self.wrapped_players.values().collect();
    }
}

#[derive(Table, Deserialize, Debug)]
pub struct Player {
    id: i32,
    active: bool,
    callsign: String,
    #[table(skip)]
    contracts: Vec<Contract>,
    last_active: SpaceTime,
    money: f32,
    ship: Ship,
    team: Team,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.callsign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn players_are_deserialized_properly() {
        let players = r#"
      {
      "1":{
      "active":false,
      "callsign":"Lee",
      "id":1,
      "last_active":{
         "nanos_since_epoch":280966047,
         "secs_since_epoch":1624646523
      },
      "contracts":{
         "371":{
            "cargo_type":"Methane",
            "current_cost":691,
            "destination":{
               "id":11,
               "name":"Neptune",
               "x_coord":81,
               "y_cookjrd":781
            },
            "expiration":{
               "nanos_since_epoch":990493345,
               "secs_since_epoch":1624582896
            },
            "id":371,
            "origin":{
               "id":7,
               "name":"Europa",
               "x_coord":430,
               "y_coord":390
            },
            "pay_rate":16,
            "starting_cost":2524,
            "volume":526
         },
         "929":{
            "cargo_type":"Iron",
            "current_cost":2218,
            "destination":{
               "id":5,
               "name":"Mars",
               "x_coord":151,
               "y_coord":151
            },
            "expiration":{
               "nanos_since_epoch":465336827,
               "secs_since_epoch":1621019751
            },
            "id":929,
            "origin":{
               "id":10,
               "name":"Uranus",
               "x_coord":737,
               "y_coord":207
            },
            "pay_rate":13,
            "starting_cost":16680,
            "volume":4277
         }
      },
      "money":46601.0,
      "ship":{
         "bearing":0.0,
         "cargo":{},
         "course":null,
         "engine_heat":0.0,
         "fuel":77.2797999999889,
         "location":{
            "id":7,
            "name":"Europa",
            "x_coord":430,
            "y_coord":390
         },
         "max_cargo":10000,
         "max_fuel":100.0,
         "next_waypoint_index":null,
         "player_id":1,
         "position":[
            430.0,
            390.0
         ],
         "speed":0.0,
         "status":"Docked"
      },
      "team":{
         "id":1,
         "name":"Blue"
      }
   }
}
        "#;

        let deserialized: Players = serde_json::from_str(&players).unwrap();

        let player_id = deserialized.players()[0].id;
        assert_eq!(player_id, 1);
    }
}

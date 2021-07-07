mod session;
use session::Session;
use std::io;
use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize, Deserializer};
use reqwest::header;
use cli_table::{format::Justify, print_stdout, Table, WithTitle};

enum Command {
    Locations,
    Contracts,
    Players,
    Exit,
    Undefined,
}


#[derive(Serialize, Deserialize, Debug)]
struct StringOption<T>(pub Option<T>);

impl<T: fmt::Display> fmt::Display for StringOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(string) => write!(f, "{}", string),
            None => write!(f, ""),
        }
    }
}

#[derive(Table, Serialize, Deserialize, Debug)]
struct Location {
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

#[derive(Table, Serialize, Deserialize, Debug)]
struct SpaceTime {
    nanos_since_epoch: i32,
    secs_since_epoch: i32,
}

impl fmt::Display for SpaceTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nanos_since_epoch)
    }
}

#[derive(Table, Serialize, Deserialize, Debug)]
struct Contract {
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

#[derive(Table, Serialize, Deserialize, Debug)]
struct Ship {
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

#[derive(Table, Serialize, Deserialize, Debug)]
struct Team {
    id: i32,
    name: String,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Deserialize, Debug)]
struct Players {
    #[serde(flatten)]
    players: HashMap<String, Player>,
}

#[derive(Table, Deserialize, Debug)]
struct Player {
    id: i32,
    active: bool,
    callsign: String,
    #[serde(skip)]
    #[table(skip)]
    contracts: HashMap<String, Contract>,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut current_session = Session::LoggedOut;
    let server_url = "https://lq-space.herokuapp.com";
    loop {
        match &current_session {
            Session::LoggedOut => {
                let result = session::create_from_user_input(server_url).await;
                match result {
                    Ok(session) => {
                        current_session = session;
                    }
                    Err(e) => {
                        println!("Problems");
                    }
                }
            }
            Session::LoggedIn { ref token } => {
                let mut user_input = String::new();
                let command: Command;
                let mut headers = header::HeaderMap::new();
                let mut auth_value = header::HeaderValue::from_str(token).expect("invalid token");
                auth_value.set_sensitive(true);
                headers.insert(header::AUTHORIZATION, auth_value);
                let client = reqwest::Client::builder()
                    .default_headers(headers)
                    .build()?;


                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");

                let parsed_user_input: Vec<&str> = user_input.trim().split(" ").collect();
                command = match parsed_user_input.len() {
                    1 => {
                        match &parsed_user_input[0][..] {
                            "locations" => Command::Locations,
                            "contracts" => Command::Contracts,
                            "players" => Command::Players,
                            "exit" => Command::Exit,
                            _ => Command::Undefined,
                        }
                    },
                    _ => Command::Undefined,
                };
                match command {
                    Command::Locations => {
                        println!("Locations:");
                        let resp = client.get(format!("{}{}", server_url, "/locations.json"))
                            .send()
                            .await?
                            .json::<Vec<Location>>()
                            .await?;
                        print_stdout(resp.with_title()).expect("Failed to fetch locations");
                    }
                    Command::Contracts => {
                        println!("Contracts:");
                        let resp = client.get(format!("{}{}", server_url, "/contracts.json"))
                            .send()
                            .await?
                            .json::<HashMap<String, Contract>>()
                            .await?;
                        print_stdout(resp.values().with_title()).expect("Failed to fetch contracts");
                    }
                    Command::Players => {
                        println!("Players:");
                        // let resp = client.get(format!("{}{}", server_url, "/players.json"))
                        //     .send()
                        //     .await?
                        //     .text()
                        //     .await?;
                        let resp = client.get(format!("{}{}", server_url, "/players.json"))
                            .send()
                            .await?
                            .json::<Vec<Player>>()
                            .await?;
                        print_stdout(resp.with_title()).expect("Failed to fetch players");
                    }
                    Command::Undefined => {
                        println!("Unknown!");
                    }
                    Command::Exit => {
                        break;
                    }
                };
            }
        }
    }
    Ok(())
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

        // Prints deserialized = Point { x: 1, y: 2 }
        println!("Hello!");
        println!("deserialized = {:?}", deserialized.players);
        assert_eq!(2 + 2, 4);
    }
}

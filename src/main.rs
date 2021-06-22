mod session;
use session::Session;
use std::io;
use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use reqwest::header;
use cli_table::{format::Justify, print_stdout, Table, WithTitle};

enum Command {
    Locations,
    Contracts,
    Players,
    Exit,
    Undefined,
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
struct Expiration {
    nanos_since_epoch: i32,
    secs_since_epoch: i32,
}

impl fmt::Display for Expiration {
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
    expiration: Expiration,
    pay_rate: i32,
    starting_cost: i32,
    volume: i32,
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
                            // .text();
                        // println!("{}", resp.await.expect("Failed"));
                        print_stdout(resp.values().with_title()).expect("Failed to fetch contracts");
                    }
                    Command::Players => {
                        println!("Get players!");
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

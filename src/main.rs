mod session;
use session::Session;
// use session::create_from_user_input;

use std::io;
// use std::collections::HashMap;
use serde::{Serialize, Deserialize};
// use reqwest::Client;
// use reqwest::StatusCode;
use reqwest::header;

enum Command {
    Locations,
    Contracts,
    Players,
    Exit,
    Undefined,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    id: i32,
    name: String,
    x_coord: i32,
    y_coord: i32,
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
                println!("{:?}", parsed_user_input);
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
                        println!("Get locations!");
                        let resp = client.get(format!("{}{}", server_url, "/locations.json"))
                            .send()
                            .await?
                            .json::<Vec<Location>>()
                            .await?;
                        println!("{:#?}", resp);
                    }
                    Command::Contracts => {
                        println!("Get contracts!");
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

use std::io;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::header;

enum Command {
    Locations,
    Contracts,
    Players,
    Exit,
    Undefined,
}

enum Session {
    LoggedIn { token: String },
    LoggedOut,
}

#[derive(Serialize, Deserialize, Debug)]
struct Login {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = Session::LoggedOut;
    let server_url = "https://lq-space.herokuapp.com";
    loop {
        match &session {
            Session::LoggedOut => {
                let mut email = String::new();
                let mut password = String::new();
                println!("Welcome to Space Hauler!");
                println!("Enter your login email");
                io::stdin()
                    .read_line(&mut email)
                    .expect("Failed to read line");
                println!("Enter your login password");
                io::stdin()
                    .read_line(&mut password)
                    .expect("Failed to read line");
                println!("Logging in {:?} {:?} !", email, password);
                let client = Client::new();
                let login = Login {
                    email: email.trim().to_string(),
                    password: password.trim().to_string(),
                };
                let req = client.post(format!("{}{}", server_url, "/sessions.json"))
                    .json(&login)
                    .send()
                    .await?;
                match req.status() {
                    StatusCode::OK => {
                        let resp = req.json::<HashMap<String, String>>().await?;
                        match resp.get("token") {
                            Some(token) => {
                                session = Session::LoggedIn { token: token.to_string() };
                                println!("Logged In!");
                            }
                            None => {
                                println!("Unable to login");
                                println!("{:#?}", resp);
                            }
                        };
                    }
                    _ => {
                        println!("Server status was {}", req.status());
                    }
                };
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
                            .json::<HashMap<String, String>>()
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

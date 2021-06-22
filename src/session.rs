// use std::io;
// use reqwest::Client;
// use reqwest::StatusCode;
use std::collections::HashMap;
// use serde::{Serialize, Deserialize};

pub enum Session {
    LoggedIn { token: String },
    LoggedOut,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Login {
    email: String,
    password: String,
}

pub async fn create_from_user_input(server_url: &str) -> Result<Session, &str> {
    let mut email = String::new();
    let mut password = String::new();
    println!("Welcome to Space Hauler!");
    println!("Enter your login email");
    std::io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");
    println!("Enter your login password");
    std::io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    println!("Logging in {:?} {:?} !", email, password);
    let client = reqwest::Client::new();
    let login = Login {
        email: email.trim().to_string(),
        password: password.trim().to_string(),
    };
    let req = client.post(format!("{}{}", server_url, "/sessions.json"))
        .json(&login)
        .send()
        .await;
    match req {
        Ok(req) => {
            match req.status() {
                reqwest::StatusCode::OK => {
                    let resp = req.json::<HashMap<String, String>>().await;
                    match resp {
                        Ok(resp) => {
                            match resp.get("token") {
                                Some(token) => {
                                    println!("Logged In!");
                                    return Ok(Session::LoggedIn { token: token.to_string() });
                                }
                                None => {
                                    return Err("Unable to login");
                                }
                            };
                        }
                        Err(e) => {
                            println!("Problem logging in.");
                            Err("Problem loggin in.")
                        }
                    }
                }
                _ => {
                    Err("Problem loggin in.")
                }
            }
        }
        Err(e) => {
            println!("Problem logging in.");
            Err("Problem loggin in.")
        }
    }
}

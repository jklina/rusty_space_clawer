mod session;
mod player;
mod team;
mod contract;
mod ship;
mod space_time;
mod location;
mod string_option;
mod post_command;
use contract::Contract;
use player::Players;
use location::Location;
use session::Session;
use post_command::PostCommand;
use std::io;
use reqwest::header;
use cli_table::{print_stdout, WithTitle};

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
                        println!("There was a problem logging in: {}", e);
                    }
                }
            }
            Session::LoggedIn { ref token } => {
                let mut user_input = String::new();
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
                // let arg_length = parsed_user_input.len();
                let command_input = parsed_user_input[0];
                let args = parsed_user_input.get(1);
                match (command_input, args) {
                    ("locations", None) => { 
                        let resp = client.get(format!("{}{}", server_url, "/locations.json"))
                            .send()
                            .await?
                            .json::<Vec<Location>>()
                            .await?;
                        print_stdout(resp.with_title()).expect("Failed to fetch locations");
                    },
                    ("contracts", None) => {
                        let resp = client.get(format!("{}{}", server_url, "/contracts.json"))
                            .send()
                            .await?
                            .json::<Vec<Contract>>()
                            .await?;
                        print_stdout(resp.with_title()).expect("Failed to fetch players");
                    },

                    ("players", None) => {
                        let resp = client.get(format!("{}{}", server_url, "/players.json"))
                            .send()
                            .await?
                            .json::<Players>()
                            .await?;
                        print_stdout(resp.players().with_title()).expect("Failed to fetch players");
                    },

                    ("launch", None) => {
                        let command = String::from("launch");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },
                    ("load_cargo", None) => {
                        let command = String::from("load_cargo");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },
                    ("unload_cargo", None) => {
                        let command = String::from("unload_cargo");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },

                    ("refuel", None) => {
                        let command = String::from("refuel");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },

                    ("stop", None) => {
                        let command = String::from("stop");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },

                    ("emergency_stop", None) => {
                        let command = String::from("emergency_stop");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },

                    ("self_destruct", None) => {
                        let command = String::from("self_destruct");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },
                    ("jettison", None) => {
                        let command = String::from("jettison");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },
                    ("dock", None) => {
                        let command = String::from("dock");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    },
                    ("accept_contract", Some(args)) => {
                        let command = String::from("accept_contract");
                        let resp = PostCommand::send(command, Some(args.to_string()), client, server_url);
                        println!("{}", resp.await.unwrap());
                    },
                    ("abort_contract", Some(args)) => {
                        let command = String::from("abort_contract");
                        let resp = PostCommand::send(command, Some(args.to_string()), client, server_url);
                        println!("{}", resp.await.unwrap());
                    },
                    ("plot_course", Some(args)) => {
                        let command = String::from("plot_course");
                        let resp = PostCommand::send(command, Some(args.to_string()), client, server_url);
                        println!("{}", resp.await.unwrap());
                    },
                    ("exit", None) => break,
                    (_,_) => println!("Unknown command, try again!"),
                };
            }
        }
    }
    Ok(())
}

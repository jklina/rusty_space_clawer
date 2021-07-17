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

enum Command {
    Locations,
    Contracts,
    Players,
    Launch,
    LoadCargo,
    UnloadCargo,
    Refuel,
    Stop,
    EmergencyStop,
    SelfDestruct,
    Jettison,
    Dock,
    AcceptContract(String),
    AbortContract(String),
    PlotCourse(String),
    Exit,
    Undefined,
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
                        println!("There was a problem logging in: {}", e);
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
                            "launch" => Command::Launch,
                            "load_cargo" => Command::LoadCargo,
                            "unload_cargo" => Command::UnloadCargo,
                            "refuel" => Command::Refuel,
                            "stop" => Command::Stop,
                            "emergency_stop" => Command::EmergencyStop,
                            "self_destruct" => Command::SelfDestruct,
                            "jettison" => Command::Jettison,
                            "dock" => Command::Dock,
                            "exit" => Command::Exit,
                            _ => Command::Undefined,
                        }
                    },
                    2 => {
                        match &parsed_user_input[0][..] {
                            "accept_contract" => Command::AcceptContract(parsed_user_input[1].to_string()),
                            "abort_contract" => Command::AbortContract(parsed_user_input[1].to_string()),
                            "plot_course" => Command::PlotCourse(parsed_user_input[1].to_string()),
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
                            .json::<Vec<Contract>>()
                            .await?;
                        print_stdout(resp.with_title()).expect("Failed to fetch players");
                    }
                    Command::Players => {
                        println!("Players:");
                        let resp = client.get(format!("{}{}", server_url, "/players.json"))
                            .send()
                            .await?
                            .json::<Players>()
                            .await?;
                        print_stdout(resp.players().with_title()).expect("Failed to fetch players");
                    }
                    Command::Launch => {
                        let command = String::from("launch");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::LoadCargo => {
                        let command = String::from("load_cargo");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::UnloadCargo => {
                        let command = String::from("unload_cargo");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::Refuel => {
                        let command = String::from("refuel");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::Stop => {
                        let command = String::from("stop");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::EmergencyStop => {
                        let command = String::from("emergency_stop");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::SelfDestruct => {
                        let command = String::from("self_destruct");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::Jettison => {
                        let command = String::from("jettison");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::Dock => {
                        let command = String::from("dock");
                        let resp = PostCommand::send(command, None, client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::AcceptContract(contract_number) => {
                        let command = String::from("accept_contract");
                        let resp = PostCommand::send(command, Some(contract_number), client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::AbortContract(contract_number) => {
                        let command = String::from("abort_contract");
                        let resp = PostCommand::send(command, Some(contract_number), client, server_url);
                        println!("{}", resp.await.unwrap());
                    }
                    Command::PlotCourse(course_string) => {
                        let command = String::from("plot_course");
                        let resp = PostCommand::send(command, Some(course_string), client, server_url);
                        println!("{}", resp.await.unwrap());
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

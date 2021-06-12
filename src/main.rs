use std::io;

enum Command<'a> {
    Login { username: &'a str, password: &'a str },
    Locations,
    Contracts,
    Players,
    Exit,
    Undefined,
}

fn main() {
    loop {
        let mut user_input = String::new();
        let command: Command;

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
            3 => {
                match &parsed_user_input[0][..] {
                    "login" => Command::Login { username: &parsed_user_input[1], password: &parsed_user_input[2] },
                    _ => Command::Undefined,
                }
            },
            _ => Command::Undefined,
        };

        match command {
            Command::Login { username, password } => {
                println!("Logging in {:?} {:?} !", username, password);
            }
            Command::Locations => {
                println!("Get locations!");
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
        }
    }
}

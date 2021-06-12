// use hyper::body::HttpBody as _;
// use hyper::Client;
// use tokio::io::{self, AsyncWriteExt as _};


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     // This is where we will setup our HTTP client requests.
//     let client = Client::new();
//
//     // Parse an `http::Uri`...
//     let uri = "http://google.com".parse()?;
//
//     // Await the response...
//     let mut resp = client.get(uri).await?;
//
//     println!("Response: {}", resp.status());
//
//     while let Some(chunk) = resp.body_mut().data().await {
//         io::stdout().write_all(&chunk?).await?;
//     }
//
//     Ok(())
// }
//
use std::env;

enum Command<'a> {
    Login { username: &'a str, password: &'a str },
    Locations,
    Contracts,
    Players,
    Undefined,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let command: Command;
    println!("{:?}", args);


    command = match args.len() {
        2 => {
            match &args[1][..] {
                "locations" => Command::Locations,
                "contracts" => Command::Contracts,
                "players" => Command::Players,
                _ => Command::Undefined,
            }
        },
        4 => {
            match &args[1][..] {
                "login" => Command::Login { username: &args[2], password: &args[3] },
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
    }
}

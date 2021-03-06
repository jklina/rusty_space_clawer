use std::collections::HashMap;
use rpassword;

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

    println!("Welcome to Space Hauler!");
    println!("Enter your login email");
    std::io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    let mut password = rpassword::prompt_password_stdout("Enter your login password\n").expect("Failed to read password");

    email = email.trim().to_string();
    password = password.trim().to_string();
    println!("Logging in {:?} !", email);
    let client = reqwest::Client::new();
    let login = Login {
        email: email,
        password: password,
    };
    let req = client.post(format!("{}{}", server_url, "/sessions.json"))
        .json(&login)
        .send()
        .await
        .unwrap();
    match req.status() {
        reqwest::StatusCode::OK => {
            let resp = req.json::<HashMap<String, String>>().await.unwrap();
            match resp.get("token") {
                Some(token) => {
                    println!("Logged In!");
                    return Ok(Session::LoggedIn { token: token.to_string() });
                }
                None => {
                    return Err("There was a problem fetching the token.");
                }
            };
        }
        _ => {
            Err("Unexpected response from server.")
        }
    }
}

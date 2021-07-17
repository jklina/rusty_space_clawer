use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PostCommand {
    command: String,
    args: Option<String>,
}

impl PostCommand {
    pub async fn send(command: String, args: Option<String>, client: reqwest::Client, server_url: &str) -> Result<String, reqwest::Error> {
        let command = PostCommand { command: command, args: args };
        return client.post(format!("{}{}", server_url, "/send_command.json"))
            .json(&command)
            .send()
            .await?
            .text()
            .await;
    }
}


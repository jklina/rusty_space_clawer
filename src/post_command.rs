use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PostCommand {
    command: String,
}

impl PostCommand {
    pub async fn send(command: String, client: reqwest::Client, server_url: &str) -> Result<String, reqwest::Error> {
        let command = PostCommand { command: command };
        return client.post(format!("{}{}", server_url, "/send_command.json"))
            .json(&command)
            .send()
            .await?
            .text()
            .await;
    }
}


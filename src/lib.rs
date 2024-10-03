use reqwest::Client;
use serde::{Deserialize, Serialize};

const TELEGRAM_API_URL: &str = "https://api.telegram.org/bot";

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub text: Option<String>,
    pub chat: Chat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
}

pub struct TelegramBot {
    token: String,
    client: Client,
}

impl TelegramBot {
    pub fn new(token: &str) -> Self {
        TelegramBot {
            token: token.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_updates(&self) -> Result<Vec<Update>, reqwest::Error> {
        let url = format!("{}/getUpdates", self.api_url());
        let response = self.client.get(&url).send().await?;

        let updates: Vec<Update> = response.json().await?;
        Ok(updates)
    }

    pub async fn send_message(&self, chat_id: i64, text: &str) -> Result<(), reqwest::Error> {
        let url = format!("{}/sendMessage", self.api_url());
        let params = [("chat_id", chat_id.to_string()), ("text", text.to_string())];

        self.client.post(&url).form(&params).send().await?;
        Ok(())
    }

    fn api_url(&self) -> String {
        format!("{}{}", TELEGRAM_API_URL, self.token)
    }
}
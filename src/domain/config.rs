#[derive(Debug, Deserialize)]
pub struct TelegramConfiguration {
    pub api_token: String
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub debug: bool,
    pub telegram: TelegramConfiguration
}
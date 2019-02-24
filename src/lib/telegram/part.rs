pub trait Part {
    fn render(&self, api: &telegram_bot::Api) -> Result<(), telegram_bot::Error>;
}

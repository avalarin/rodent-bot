use super::part::*;
use telegram_bot::Error;
use telegram_bot::Api;

pub struct RequestPart {
    closure: Box<Fn(&telegram_bot::Api)>,
}

impl RequestPart {
    pub fn new<R: telegram_bot::Request + 'static>(request: R) -> Self {
        let closure = move |api: &telegram_bot::Api| {
            api.spawn(&request)
        };

        RequestPart {
            closure: Box::new(closure)
        }
    }
}

impl Part for RequestPart {
    fn render(&self, api: &Api) -> Result<(), Error> {
        (self.closure)(api);
        Ok(())
    }
}

impl <R: telegram_bot::Request + 'static> From<R> for Box<Part> {
    fn from(req: R) -> Self {
        Box::new(RequestPart::new(req))
    }
}
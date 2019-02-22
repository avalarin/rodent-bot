extern crate config;
extern crate serde;
extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate fern;

pub mod app;
pub mod domain;
pub mod lib;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

use domain::pipeline::Pipelines;
use domain::context::Context;
use std::sync::Arc;

fn main() {
    let config = self::app::config::load_config().expect("Cannot load configuration");

    self::app::logging::setup_logging(config.debug).expect("Cannot configure logging engine");

    let mut core = Core::new().unwrap();
    let api = Arc::new(Api::configure(config.telegram.api_token).build(core.handle()).unwrap());
    let pipeline = Pipelines::create();

    let future = api.stream().for_each(|update| {
        let _result = pipeline.call(Context{
            api: api.clone(),
            update
        });
        Ok(())
    });

    info!("Bot application has been started.");
    core.run(future).unwrap();
}

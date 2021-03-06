extern crate config;
extern crate serde;
extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate chrono;
extern crate rand;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate fern;
#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate diesel;

pub mod app;
pub mod domain;
pub mod lib;
pub mod schema;

use std::sync::Arc;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

use domain::pipeline::Pipelines;
use domain::context::Context;
use domain::services::{
    users::UsersServiceImpl,
    context::ContextServiceImpl
};

fn main() {
    let config = self::app::config::load_config().expect("Cannot load configuration");

    self::app::logging::setup_logging(config.debug).expect("Cannot configure logging engine");

    let mut core = Core::new().unwrap();
    let api = Arc::new(Api::configure(config.telegram.api_token).build(core.handle()).unwrap());

    let db = Arc::new(self::app::db::DB::new(config.postgres));
    let users = Arc::new(UsersServiceImpl::new(db.clone()));
    let context = Arc::new(ContextServiceImpl::new(db.clone()));
    let pipeline = Pipelines::create(users, context, api.clone());

    let future = api.stream().for_each(|update| {
        let _ = pipeline.call(Context::new(update))
            .map_err(|err| {
                error!("Uncaught error: {}", err);
                err
            });
        Ok(())
    });

    info!("Bot application has been started.");
    core.run(future).unwrap();
}

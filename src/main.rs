#![feature(return_position_impl_trait_in_trait)]

extern crate log;
extern crate pretty_env_logger;

mod client;
mod database;
mod server;

// use database::update_database;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::{env, error};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    pretty_env_logger::init();

    let client = get_client("mongodb://localhost:27017").await;

    let db = client.database("schedule");
    let routes = server::get_filters(db).with(warp::log("server"));
    // should never return
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

/// get_client(url: ToString) -> Client
/// Setup MongoDB Client
async fn get_client<T: ToString>(url: T) -> Client {
    let url = url.to_string();
    let mut options = ClientOptions::parse(url).await.unwrap();
    options.app_name = Some("Schedule App".to_string());
    Client::with_options(options).unwrap() // .database("databasename")
}

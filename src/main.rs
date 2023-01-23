mod client;
mod database;
mod server;

use chrono::prelude::*;
use tokio::sync::watch;
use std::{env, error};
use warp::Filter;

#[tokio::main]
#[rorm::rorm_main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    pretty_env_logger::init();

    let (from, to) = client::get_current_week(Utc::now()).unwrap();

    let routes = server::get_filters().with(warp::log("server"));
    // should never return
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

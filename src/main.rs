mod client;
mod server;

use std::collections::HashMap;
use std::{env, error};
use std::fmt::{Display, Error, Formatter};
use std::time::SystemTime;

use chrono::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use warp::Filter;



#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }
    pretty_env_logger::init();

    let routes = server::get_filters()
        .with(warp::log("server"));
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030));
    Ok(())
}



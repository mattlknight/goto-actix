#![warn(rust_2018_idioms)]

use libgoto::server;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "goto=debug,libgoto=debug,diesel=debug,actix_web=trace");
    env_logger::init();
    server::start();
}

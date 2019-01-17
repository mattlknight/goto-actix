#![warn(rust_2018_idioms)]

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    libgoto::server::start();
}

use dotenv::dotenv;
use actix_web::{server, pred, App};
use actix_web::http::{Method};
use log::{debug, warn};
use std::net::SocketAddr;
use std::env;
// use crate::db;
use crate::routes;

fn get_socket() -> SocketAddr {
    let port = match env::var("GOTO_PORT") {
        Ok(port) => port,
        Err(e) => {
            warn!("Error reading GOTO_PORT from env, {:?}", e);
            "8080".to_owned()
        }
    };
    let address = match env::var("GOTO_ADDRESS") {
        Ok(address) => address,
        Err(e) => {
            warn!("Error reading GOTO_ADDRESS from env, {:?}", e);
            "127.0.0.1".to_owned()
        }
    };
    let socket: SocketAddr = format!("{}:{}", address, port).parse().expect("Failed to build socket from GOTO_ADDRESS and GOTO_PORT env vars");
    socket
}

pub fn start() {
    debug!("Starting GoTo Server");

    dotenv().ok();

    // let connection = db::establish_connection();
    // db::print_keywords(connection);

    let socket = get_socket();
    debug!("Binding to {}", socket);

    server::new(|| {
        App::new()
            .resource("/", |r| r.method(Method::GET).f(routes::get_index))
            .resource("/favicon.ico", |r| r.f(routes::favicon))
            .resource("/error.html", |r| r.f(routes::get_error))

            .resource("/api/keyword", |r| {
                r.route().filter(pred::Get()).with(routes::get_keywords);
                r.route().filter(pred::Post()).f(routes::add_keyword);
            })
            .resource("/api/keyword/{keyword}", |r| {
                r.route().filter(pred::Get()).with(routes::get_keyword);
                r.route().filter(pred::Put()).with(routes::update_keyword);
                r.route().filter(pred::Delete()).with(routes::delete_keyword);
            })
            .resource("/{keyword}", |r| r.method(Method::GET).with(routes::redirect_keyword))
    })
    .bind(socket)
    .expect(&format!("Can not bind to {:?}", socket))
    .run();
}

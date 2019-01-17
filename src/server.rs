use actix_web::{server, App};
use actix::SyncArbiter;
use crate::db;
use crate::routes;
use crate::types::AppState;
use dotenv::dotenv;
use log::{debug, warn};
use std::env;
use std::net::SocketAddr;

fn get_socket() -> SocketAddr {
    let port = match env::var("GOTO_PORT") {
        Ok(port) => port,
        Err(e) => {
            warn!("Error reading GOTO_PORT from env, {:?}", e);
            "8888".to_owned()
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
    dotenv().ok();

    debug!("Starting GoTo Server");

    let socket = get_socket();
    debug!("Binding to {}", socket);

    let sys = actix::System::new("diesel-system");
    let addr = SyncArbiter::start(4, || {
        db::DbCon(db::establish_connection())
    });

    db::print_keywords(db::establish_connection());

    server::HttpServer::new(move || {
        App::with_state(AppState{db: addr.clone()})
            .resource("/api/keyword", |r| {
                r.get().with_async(routes::get_keywords);
                r.post().with_async(routes::post_keyword);
            })
            .resource("/api/keyword/{keyword}", |r| {
                r.get().with_async(routes::get_keyword);
                r.put().with_async(routes::put_keyword);
                r.delete().with_async(routes::delete_keyword);
            })
            .resource("/{keyword}", |r| r.get().with(routes::redirect_keyword))
    })
    .bind(socket)
    .expect(&format!("Can not bind to {:?}", socket))
    .start();

    debug!("Started http server: {:?}", socket);
    let _ = sys.run();
}

use actix_web::{server, App};
use actix_web::middleware::Logger;
use actix::SyncArbiter;
use crate::db;
use crate::routes;
pub use crate::types::{AppState, RequestInterceptor};
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

pub fn build_state() -> AppState {
    let addr = SyncArbiter::start(4, || {
        db::DbCon(db::establish_connection())
    });

    AppState{db: addr.clone()}
}

fn create_app() -> App<AppState> {
    App::with_state(build_state())
        .middleware(RequestInterceptor)
        .middleware(Logger::default())
        .middleware(Logger::new("%a %{User-Agent}i"))
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
}

pub fn start() {
    dotenv().ok();

    debug!("Starting GoTo Server");

    let socket = get_socket();
    debug!("Binding to {}", socket);

    let sys = actix::System::new("diesel-system");

    db::print_keywords(db::establish_connection());

    server::HttpServer::new(create_app)
        .bind(socket)
        .expect(&format!("Can not bind to {:?}", socket))
        .start();

    debug!("Started http server: {:?}", socket);
    let _ = sys.run();
}

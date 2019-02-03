use actix_web::{server, fs, http, App};
use actix_web::middleware::Logger;
use actix_web::middleware::cors::Cors;
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
    let app = App::with_state(build_state());
    let app = Cors::for_app(app)
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
        .resource("/api/doc", |r| r.get().f(routes::get_swagger_index))
        .resource("/api/doc/swagger.yml", |r| r.get().f(routes::get_swagger_yml))
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
        .register()
        .handler("/api/doc", fs::StaticFiles::new("swagger-ui/dist/").unwrap())
        .middleware(RequestInterceptor)
        .middleware(Logger::default())
        .middleware(Logger::new("%a %{User-Agent}i"));

    #[cfg(debug_assertions)]
    let app = app.resource("/api/editor", |r| r.get().f(routes::get_swagger_editor_index))
        .resource("/api/editor/swagger.yml", |r| r.get().f(routes::get_swagger_yml))
        .handler("/api/editor", fs::StaticFiles::new("swagger-editor/").unwrap());

    app
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

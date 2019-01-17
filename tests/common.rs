use actix_web::test::TestServer;
use actix_web::middleware::Logger;
use libgoto::{routes, server, establish_connection};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl, BoolExpressionMethods};
use log::debug;

pub fn start_test_server() -> TestServer {
    dotenv::dotenv().ok();
    env_logger::init();

    TestServer::build_with_state(server::build_state)
        .start(|app| {
            app.middleware(server::RequestInterceptor)
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
            .resource("/{keyword}", |r| r.get().with(routes::redirect_keyword));
        })
}

pub fn reset_db() {
    dotenv::dotenv().ok();
    let db = establish_connection();
    use libgoto::schema::keywords::dsl::*;
    match diesel::delete(keywords.filter(keyword.eq("thisisatest").or(keyword.eq("thisisatest2")))).execute(&db) {
        Ok(num_rows_affected) => debug!("Reset DB deleted {} rows", num_rows_affected),
        Err(err) => panic!("Reset DB {:?}", err),
    }
}

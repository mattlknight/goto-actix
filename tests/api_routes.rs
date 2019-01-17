use actix_web::client::{ClientRequest};
use actix_web::http;
use actix_web::http::header;
use actix_web::test::TestServer;

mod common;

fn delete_request(srv: &mut TestServer) -> ClientRequest {
    srv.client(
         http::Method::DELETE, "/api/keyword/thisisatest")
         .finish().expect("Failed to create request")
}

fn delete_request2(srv: &mut TestServer) -> ClientRequest {
    srv.client(
         http::Method::DELETE, "/api/keyword/thisisatest2")
         .finish().expect("Failed to create request")
}

fn post_request(srv: &mut TestServer) -> ClientRequest {
    srv.client(
         http::Method::POST, "/api/keyword")
         .header(header::CONTENT_TYPE, "application/json")
         .body(include_str!("post.json"))
         .expect("Failed to create Json request")
}

#[test]
fn test_crud() {
    common::reset_db();
    let mut srv = common::start_test_server();

    let request = post_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent initial POST, expected SUCCESS");

    let request = post_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_server_error(), "Sent duplicate POST, expected server error 500-599");

    let request = delete_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent DELETE request for 'thisisatest', expected SUCCESS always");

    let request = delete_request2(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent DELETE request for 'thisisatest2', expected SUCCESS always");

    common::reset_db();
}

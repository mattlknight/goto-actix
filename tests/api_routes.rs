use actix_web::client::{ClientRequest};
use actix_web::http;
use actix_web::http::header;
use actix_web::test::TestServer;

mod common;

fn post_request(srv: &mut TestServer) -> ClientRequest {
    srv.client(
         http::Method::POST, "/api/keyword")
         .header(header::CONTENT_TYPE, "application/json")
         .body(include_str!("post.json"))
         .expect("Failed to create Json request")
}

fn get_request(srv: &mut TestServer) -> ClientRequest {
    srv.client(
         http::Method::GET, "/api/keyword/thisisatest")
         .finish()
         .expect("Failed to create Json request")
}

fn put_request(srv: &mut TestServer) -> ClientRequest {
    srv.client(
         http::Method::PUT, "/api/keyword/thisisatest")
         .header(header::CONTENT_TYPE, "application/json")
         .body(include_str!("put.json"))
         .expect("Failed to create Json request")
}

fn get_many_request(srv: &mut TestServer) -> ClientRequest {
    srv.client(
         http::Method::GET, "/api/keyword/thisisatest")
         .header(header::CONTENT_TYPE, "application/json")
         .body(include_str!("get.json"))
         .expect("Failed to create Json request")
}


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


#[test]
fn test_crud() {
    common::reset_db();
    let mut srv = common::start_test_server();

    let request = post_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent initial POST, expected SUCCESS");

    let request = post_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_client_error(), "Sent duplicate POST, expected CLIENT ERROR");

    let request = get_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent GET request for 'thisisatest', expected SUCCESS always");

    let request = get_many_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent GET MANY request, expected SUCCESS always");

    let request = put_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent PUT request for 'thisisatest', expected SUCCESS");

    let request = delete_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent DELETE request for 'thisisatest', expected SUCCESS always");

    let request = put_request(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_client_error(), "Sent PUT request for 'thisisatest', expected CLIENT ERROR");

    let request = delete_request2(&mut srv);
    let response = srv.execute(request.send()).expect("Failed to execute request send");
    assert!(response.status().is_success(), "Sent DELETE request for 'thisisatest2', expected SUCCESS always");

    common::reset_db();
}

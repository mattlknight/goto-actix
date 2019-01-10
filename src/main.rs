use actix_web::{server, App, HttpRequest};

fn main() {
    server::new(|| App::new()
                .resource("/", |r| r.f(index))
                .resource("/yourmom", |r| r.f(mom))
                .resource("/friend", |r| r.f(friend))
                .resource("/guy", |r| r.f(guy))
                .resource("/buddy", |r| r.f(buddy)))
        .bind("127.0.0.1:80")
        .unwrap()
        .run();
}

fn index(_req: &HttpRequest) -> &'static str {
    "This is an index!"
}

fn mom(_req: &HttpRequest) -> &'static str {
    "Go Fund Yourself!"
}

fn friend(_req: &HttpRequest) -> &'static str {
    "I'm not your fwend, guy!"
}

fn guy(_req: &HttpRequest) -> &'static str {
    "I'm not your guy, buddy!"
}

fn buddy(_req: &HttpRequest) -> &'static str {
    "I'm not your buddy, fwend!"
}

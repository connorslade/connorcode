use afire::{extensions::RouteShorthands, Server};

use crate::app::App;

struct Request {
    ip: String,
    page: String,
    referrer: String,
}

pub fn attach(server: &mut Server<App>) {
    server.post("/api/analytics", |ctx| {
        //todo
        Ok(())
    });
}

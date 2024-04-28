use afire::{
    extensions::{RealIp, RouteShorthands},
    Content, Server,
};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/ip", |ctx| {
        Ok(ctx.text(ctx.req.real_ip()).content(Content::TXT).send()?)
    });
}

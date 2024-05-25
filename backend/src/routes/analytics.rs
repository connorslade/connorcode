use std::net::{IpAddr, Ipv4Addr};

use afire::{
    extensions::{RealIp, RouteShorthands},
    route::RouteContext,
    Content, Server,
};
use serde::Deserialize;
use ureq::json;

use crate::app::App;

#[derive(Debug)]
pub struct Analytics {
    pub ip: Ipv4Addr,
    pub page: String,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Deserialize)]
struct Request {
    page: String,
    referrer: Option<String>,
    user_agent: Option<String>,
}

pub fn attach(server: &mut Server<App>) {
    server.post("/api/analytics", |ctx| {
        let IpAddr::V4(ip) = ctx.real_ip() else {
            return None.context("Ipv6 addresses are not supported")?;
        };

        let data = serde_json::from_slice::<Request>(&ctx.req.body)?.into_analytics(ip);
        dbg!(&data);

        ctx.content(Content::JSON)
            .text(json!({"status": "ok"}))
            .send()?;

        ctx.app().database.insert_analytics(data)?;

        Ok(())
    });
}

impl Request {
    pub fn into_analytics(self, ip: Ipv4Addr) -> Analytics {
        Analytics {
            ip,
            page: self.page,
            referrer: self.referrer,
            user_agent: self.user_agent,
        }
    }
}

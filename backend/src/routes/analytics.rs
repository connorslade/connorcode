use std::{
    net::{IpAddr, Ipv4Addr},
    time::SystemTime,
};

use afire::{
    extensions::{RealIp, RouteShorthands},
    Content, Server,
};
use serde::Deserialize;
use tracing::error;
use ureq::json;

use crate::{app::App, misc::util::bail};

#[derive(Debug)]
pub struct Analytics {
    pub timestamp: u64,
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
            return bail("Ipv6 addresses are not supported");
        };

        let data = serde_json::from_slice::<Request>(&ctx.req.body)?.into_analytics(ip);

        ctx.content(Content::JSON)
            .text(json!({"status": "ok"}))
            .send()?;

        if let Err(err) = ctx.app().database.insert_analytics(data) {
            error!("Analytics error: {:?}", err);
        }

        Ok(())
    });
}

impl Request {
    pub fn into_analytics(self, ip: Ipv4Addr) -> Analytics {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Analytics {
            ip,
            timestamp,
            page: self.page,
            referrer: self.referrer,
            user_agent: self.user_agent,
        }
    }
}

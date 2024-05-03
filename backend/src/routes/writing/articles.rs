use afire::{extensions::RouteShorthands, Content, Server};
use serde_json::json;

use crate::{app::App, writing::article::ArticleApiResponse};

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/articles", |ctx| {
        let articles = &ctx.app().writing.articles;
        let mut out = Vec::new();

        for article in articles {
            out.push(json!(ArticleApiResponse::from_document(article)));
        }

        ctx.content(Content::JSON).text(json!(out)).send()?;
        Ok(())
    });
}

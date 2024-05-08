use afire::{extensions::RouteShorthands, Content, Server};
use serde_json::json;

use crate::{
    app::App,
    writing::{article::ArticleApiResponse, project::ProjectApiResponse},
};

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/list", |ctx| {
        let mut out = Vec::new();
        for article in &ctx.app().writing.read().articles {
            out.push(json!(ArticleApiResponse::from_document(article)));
        }

        ctx.content(Content::JSON).text(json!(out)).send()?;
        Ok(())
    });

    server.get("/api/projects/list", |ctx| {
        let mut out = Vec::new();
        for project in &ctx.app().writing.read().projects {
            out.push(json!(ProjectApiResponse::from_document(project)));
        }

        ctx.content(Content::JSON).text(json!(out)).send()?;
        Ok(())
    });
}

use std::{fs::File, path::PathBuf, sync::Arc};

use afire::{extensions::RouteShorthands, Content, Server};
use anyhow::Context;
use serde_json::json;

use crate::{app::App, writing::Article};

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/article/**", |ctx| {
        let app = ctx.app();
        let article_path = &ctx.req.path[21..];
        let article = lookup_article(&app, article_path).context("Article not found")?;

        let base_path = PathBuf::from(".writing_cache");
        let rendered = File::open(
            base_path
                .join(&article.filesystem_path)
                .with_extension("html"),
        )?;
        ctx.content(Content::HTML).stream(rendered).send()?;
        Ok(())
    });

    server.get("/api/writing/article/info/**", |ctx| {
        let app = ctx.app();
        let article_path = &ctx.req.path[26..];
        let article = lookup_article(&app, article_path).context("Article not found")?;

        ctx.content(Content::JSON)
            .text(json!(article.into_api_response()))
            .send()?;
        Ok(())
    });
}

fn lookup_article<'a>(app: &'a Arc<App>, path: &str) -> Option<&'a Article> {
    app.writing
        .articles
        .iter()
        .find(|x| x.front_matter.path == path)
}

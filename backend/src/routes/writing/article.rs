use std::{fs::File, path::PathBuf};

use afire::{extensions::RouteShorthands, route::RouteContext, Content, Server};
use serde_json::json;

use crate::{app::App, writing::article::ArticleApiResponse};

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/article/{category}/{article}", |ctx| {
        let app = ctx.app();
        let article_path = format!("{}/{}", ctx.param("category"), ctx.param("article"));
        let article = app
            .writing
            .find_article(&article_path)
            .context("Article not found")?;

        let base_path = PathBuf::from(".writing_cache");
        let rendered = File::open(
            base_path
                .join(&article.filesystem_path)
                .with_extension("html"),
        )?;
        ctx.content(Content::HTML).stream(rendered).send()?;
        Ok(())
    });

    server.get("/api/writing/info/{category}/{article}", |ctx| {
        let app = ctx.app();
        let article_path = format!("{}/{}", ctx.param("category"), ctx.param("article"));
        let article = app
            .writing
            .find_article(&article_path)
            .context("Article not found")?;

        ctx.content(Content::JSON)
            .text(json!(ArticleApiResponse::from_document(article)))
            .send()?;
        Ok(())
    });
}

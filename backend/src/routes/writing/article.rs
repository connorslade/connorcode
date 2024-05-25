use std::{fs::File, path::PathBuf};

use afire::{extensions::RouteShorthands, route::RouteContext, Content, Server};
use serde_json::json;

use crate::{
    app::App,
    writing::{article::ArticleApiResponse, project::ProjectApiResponse},
};

pub fn attach(server: &mut Server<App>) {
    {
        server.get("/api/writing/article/{category}/{article}", |ctx| {
            let app = ctx.app();
            let writing = app.writing.read();

            let article = writing
                .find_article(ctx.param("category"), ctx.param("article"))
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
            let writing = app.writing.read();

            let (category, article_name) = (ctx.param("category"), ctx.param("article"));
            let article = writing
                .find_article(category, article_name)
                .context("Article not found")?;

            let views = app
                .database
                .get_view_count(&format!("writing/{category}/{article_name}"))?;
            ctx.content(Content::JSON)
                .text(json!(ArticleApiResponse::from_document_with_views(
                    article, views
                )))
                .send()?;
            Ok(())
        });
    }

    {
        server.get("/api/writing/article/{category}", |ctx| {
            let app = ctx.app();
            let writing = app.writing.read();

            let article = writing
                .find_article_category(ctx.param("category"))
                .context("Article category not found")?;

            let base_path = PathBuf::from(".writing_cache");
            let rendered = File::open(
                base_path
                    .join(&article.filesystem_path)
                    .with_extension("html"),
            )?;
            ctx.content(Content::HTML).stream(rendered).send()?;
            Ok(())
        });

        server.get("/api/writing/info/{category}", |ctx| {
            let app = ctx.app();
            let writing = app.writing.read();

            let category = ctx.param("category");
            let article = writing
                .find_article_category(category)
                .context("Article category not found")?;

            let views = app
                .database
                .get_view_count(&format!("writing/{category}"))?;
            ctx.content(Content::JSON)
                .text(json!(ArticleApiResponse::from_document_with_views(
                    article, views
                )))
                .send()?;
            Ok(())
        });
    }

    {
        server.get("/api/projects/article/{project}", |ctx| {
            let app = ctx.app();
            let writing = app.writing.read();

            let project = writing
                .find_project(ctx.param("project"))
                .context("Project not found")?;

            let base_path = PathBuf::from(".writing_cache");
            let rendered = File::open(
                base_path
                    .join(&project.filesystem_path)
                    .with_extension("html"),
            )?;
            ctx.content(Content::HTML).stream(rendered).send()?;
            Ok(())
        });

        server.get("/api/projects/info/{project}", |ctx| {
            let app = ctx.app();
            let writing = app.writing.read();

            let project = ctx.param("project");
            let article = writing.find_project(project).context("Project not found")?;

            let views = app
                .database
                .get_view_count(&format!("projects/{project}"))?;
            ctx.content(Content::JSON)
                .text(json!(ProjectApiResponse::from_document_with_views(
                    article, views
                )))
                .send()?;
            Ok(())
        });
    }
}

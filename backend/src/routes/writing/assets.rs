use std::fs::File;

use afire::{extensions::RouteShorthands, route::RouteContext, HeaderName, Server};

use crate::{app::App, misc::mime::get_content_type};

use super::lookup_article;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/assets/{category}/{article}/**", |ctx| {
        let slug = format!("{}/{}", ctx.param("category"), ctx.param("article"));
        let file_path = &ctx.req.path[21 + slug.len()..];

        let app = ctx.app();
        let article = lookup_article(&app, &slug).context("Article not found")?;
        let relative_path = article.filesystem_path.parent().unwrap().join(file_path);
        let path = app.config.writing_path.join("writing").join(relative_path);

        let mime = get_content_type(path.extension().map(|x| x.to_string_lossy()).as_deref())
            .unwrap_or("application/octet-stream");
        let file = File::open(path)?;
        ctx.header((HeaderName::ContentType, mime))
            .stream(file)
            .send()?;
        Ok(())
    });
}

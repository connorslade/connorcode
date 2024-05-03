use std::{fs::File, path::PathBuf};

use afire::{
    error::AnyResult, extensions::RouteShorthands, route::RouteContext, Context, HeaderName, Server,
};

use crate::{app::App, misc::mime::get_content_type};

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/assets/{category}/{article}/**", |ctx| {
        let app = ctx.app();
        let (category, slug) = (ctx.param("category"), ctx.param("article"));
        let file_path = &ctx.req.path[21 + category.len() + slug.len()..];

        let article = app
            .writing
            .find_article(category, slug)
            .context("Article not found")?;
        let relative_path = article.filesystem_path.parent().unwrap().join(file_path);
        let path = app.config.writing_path.join("writing").join(relative_path);

        send_asset(path, ctx)
    });

    server.get("/api/projects/assets/{project}/**", |ctx| {
        let app = ctx.app();
        let slug = ctx.param("project");
        let file_path = &ctx.req.path[20 + slug.len()..];

        let project = app
            .writing
            .find_project(slug)
            .context("Project not found")?;
        let relative_path = project.filesystem_path.parent().unwrap().join(file_path);
        let path = app.config.writing_path.join("projects").join(relative_path);

        send_asset(path, ctx)
    });
}

fn send_asset(path: PathBuf, ctx: &Context<App>) -> AnyResult {
    let mime = get_content_type(path.extension().map(|x| x.to_string_lossy()).as_deref())
        .unwrap_or("application/octet-stream");
    let file = File::open(path)?;
    ctx.header((HeaderName::ContentType, mime))
        .stream(file)
        .send()?;
    Ok(())
}

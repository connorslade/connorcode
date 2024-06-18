use afire::{extensions::RouteShorthands, route::RouteContext, Content, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/redirect/{slug}", |ctx| {
        let slug = ctx.param("slug");

        let app = ctx.app();
        let url = app.redirects.get(slug).context("Redirect not found")?;

        ctx.text(url).content(Content::TXT).send()?;
        Ok(())
    });
}

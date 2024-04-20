use std::fs::File;

use afire::{extensions::RouteShorthands, HeaderName, Server};

use crate::{app::App, misc::mime::get_content_type};

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/assets/**", |ctx| {
        let app = ctx.app();
        let path = app
            .config
            .writing_path
            .join("assets")
            .join(&ctx.req.path[20..]);

        let mime = get_content_type(path.extension().map(|x| x.to_string_lossy()).as_deref())
            .unwrap_or("application/octet-stream");
        let file = File::open(path)?;
        ctx.header((HeaderName::ContentType, mime))
            .stream(file)
            .send()?;
        Ok(())
    });
}

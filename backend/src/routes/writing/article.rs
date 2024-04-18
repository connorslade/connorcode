use std::{fs::File, path::PathBuf};

use afire::{extensions::RouteShorthands, Content, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/article/**", |ctx| {
        let article_path = &ctx.req.path[21..];

        let base_path = PathBuf::from(".writing_cache");
        let rendered = File::open(dbg!(base_path.join(article_path).with_extension("html")))?;
        ctx.content(Content::HTML).stream(rendered).send()?;
        Ok(())
    })
}

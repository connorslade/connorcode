use std::sync::Arc;

use afire::Server;

use crate::{app::App, writing::Article};

mod article;
mod articles;
mod assets;

pub fn attach(server: &mut Server<App>) {
    articles::attach(server);
    article::attach(server);
    assets::attach(server);
}

fn lookup_article<'a>(app: &'a Arc<App>, path: &str) -> Option<&'a Article> {
    app.writing
        .articles
        .iter()
        .find(|x| x.front_matter.path == path)
}

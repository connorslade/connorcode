use afire::Server;

use crate::app::App;

mod article;
mod articles;
mod assets;

pub fn attach(server: &mut Server<App>) {
    articles::attach(server);
    article::attach(server);
    assets::attach(server);
}

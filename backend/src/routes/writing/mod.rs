use afire::Server;

use crate::app::App;

mod article;
mod articles;

pub fn attach(server: &mut Server<App>) {
    articles::attach(server);
    article::attach(server);
}

use afire::Server;

use crate::app::App;

pub mod analytics;
mod downloads;
mod files;
mod ip;
mod writing;

pub fn attach(server: &mut Server<App>) {
    analytics::attach(server);
    downloads::attach(server);
    files::attach(server);
    ip::attach(server);
    writing::attach(server);
}

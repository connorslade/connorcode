use afire::Server;

use crate::app::App;

mod analytics;
mod files;
mod writing;

pub fn attach(server: &mut Server<App>) {
    analytics::attach(server);
    files::attach(server);
    writing::attach(server);
}

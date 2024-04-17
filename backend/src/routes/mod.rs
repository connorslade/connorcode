use afire::Server;

use crate::app::App;

mod files;
mod writing;

pub fn attach(server: &mut Server<App>) {
    files::attach(server);
    writing::attach(server);
}

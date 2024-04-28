use afire::Server;

use crate::app::App;

mod files;
mod ip;
mod writing;

pub fn attach(server: &mut Server<App>) {
    files::attach(server);
    ip::attach(server);
    writing::attach(server);
}

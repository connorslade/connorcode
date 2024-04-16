use afire::Server;

use crate::app::App;

mod files;

pub fn attach(server: &mut Server<App>) {
    files::attach(server);
}

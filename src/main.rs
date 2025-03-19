mod lib;

use std::io::{BufRead, BufReader};
use lib::server::Server;

fn main() {
    let server = Server::new().port(2525);

    server.listen();
}

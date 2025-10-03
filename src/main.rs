mod connection;
mod display;
mod messages;
mod receiver;

use connection::connect_to_vehicle;
use std::env;

fn main() {
    let connection_string = parse_args().unwrap_or_else(|| {
        std::process::exit(1);
    });

    let vehicle = connect_to_vehicle(&connection_string).unwrap_or_else(|_| {
        std::process::exit(1);
    });

    messages::initialize(&vehicle);
    messages::start_heartbeat(&vehicle);
    receiver::run(&vehicle);
}

fn parse_args() -> Option<String> {
    env::args().nth(1)
}

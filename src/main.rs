mod connection;
mod display;
mod messages;
mod receiver;

use std::{env, process};

fn main() {
    let connection_string = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Error: Missing connection string argument");
        process::exit(1);
    });

    let vehicle = connection::connect_to_vehicle(&connection_string)
        .unwrap_or_else(|_| process::exit(1));

    messages::initialize(&vehicle);
    messages::start_heartbeat(&vehicle);
    receiver::run(&vehicle);
}

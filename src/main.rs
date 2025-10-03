mod connection;
mod display;
mod messages;
mod receiver;

use clap::Parser;
use std::{collections::HashSet, process};

#[derive(Parser)]
#[command(name = "mavlink_px4_rs_hello")]
#[command(about = "MAVLink PX4 message receiver and display tool")]
struct Args {
    /// Connection string (e.g., udpin:0.0.0.0:14550)
    connection: String,

    /// Filter messages to display (comma-separated, e.g., HEARTBEAT,ATTITUDE,GPS_RAW_INT)
    #[arg(long, value_delimiter = ',')]
    messages: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    let filter = build_message_filter(args.messages);
    let vehicle = connect_or_exit(&args.connection);

    messages::initialize(&vehicle);
    messages::start_heartbeat(&vehicle);
    receiver::run(&vehicle, filter);
}

fn build_message_filter(messages: Option<Vec<String>>) -> Option<HashSet<String>> {
    messages.map(|msgs| {
        msgs.into_iter()
            .map(|s| s.to_uppercase().trim().to_string())
            .collect()
    })
}

fn connect_or_exit(connection_string: &str) -> connection::MavConn {
    connection::connect_to_vehicle(connection_string).unwrap_or_else(|_| process::exit(1))
}

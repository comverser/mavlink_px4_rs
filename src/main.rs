mod connection;
mod display;
mod messages;
mod receiver;

use clap::Parser;
use std::{collections::HashSet, process};

#[derive(Parser)]
#[command(name = "mavlink_px4_rs_hello")]
#[command(about = "MAVLink message viewer for PX4", long_about = None)]
struct Args {
    /// Connection string (e.g., udpin:0.0.0.0:14550)
    connection: String,

    /// Filter messages to display (comma-separated, e.g., HEARTBEAT,ATTITUDE,GPS_RAW_INT)
    #[arg(short, long, value_delimiter = ',')]
    messages: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    let filter: Option<HashSet<String>> = args.messages.map(|msgs| {
        msgs.into_iter()
            .map(|s| s.to_uppercase().trim().to_string())
            .collect()
    });

    let vehicle = connection::connect_to_vehicle(&args.connection)
        .unwrap_or_else(|_| process::exit(1));

    messages::initialize(&vehicle);
    messages::start_heartbeat(&vehicle);
    receiver::run(&vehicle, filter);
}

use crate::{connection::MavConn, display};
use mavlink::error::MessageReadError;
use std::{collections::HashSet, io::ErrorKind, thread, time::Duration};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Main message receiver loop
pub fn run(vehicle: &MavConn, filter: Option<HashSet<String>>) {
    println!("Listening for MAVLink messages...\n");

    loop {
        match vehicle.recv() {
            Ok((header, msg)) => display::show(&header, &msg, &filter),
            Err(e) if should_retry(&e) => thread::sleep(POLL_INTERVAL),
            Err(e) if is_connection_lost(&e) => {
                eprintln!("\n✗ Connection lost: {e}");
                break;
            }
            Err(_) => {} // Skip malformed messages
        }
    }
}

fn should_retry(error: &MessageReadError) -> bool {
    matches!(error, MessageReadError::Io(e) if e.kind() == ErrorKind::WouldBlock)
}

fn is_connection_lost(error: &MessageReadError) -> bool {
    matches!(error, MessageReadError::Io(_))
}

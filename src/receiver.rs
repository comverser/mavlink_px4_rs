use crate::{connection::MavConn, display};
use mavlink::error::MessageReadError;
use std::{collections::HashSet, io::ErrorKind, thread, time::Duration};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

pub fn run(vehicle: &MavConn, filter: Option<HashSet<String>>) {
    println!("Listening for MAVLink messages...\n");

    loop {
        match vehicle.recv() {
            Ok((header, msg)) => display::show(&header, &msg, &filter),
            Err(MessageReadError::Io(e)) if e.kind() == ErrorKind::WouldBlock => {
                thread::sleep(POLL_INTERVAL);
            }
            Err(MessageReadError::Io(e)) => {
                eprintln!("\n✗ Connection lost: {e}");
                break;
            }
            Err(_) => {} // Skip malformed messages
        }
    }
}

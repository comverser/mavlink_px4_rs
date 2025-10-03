use crate::connection::MavConn;
use crate::display;
use mavlink::error::MessageReadError;
use std::{io::ErrorKind, thread, time::Duration};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

pub fn run(vehicle: &MavConn) {
    println!("Listening for MAVLink messages...\n");

    loop {
        match vehicle.recv() {
            Ok((header, msg)) => display::show(&header, &msg),
            Err(MessageReadError::Io(e)) if e.kind() == ErrorKind::WouldBlock => {
                thread::sleep(POLL_INTERVAL);
            }
            Err(MessageReadError::Io(e)) => {
                eprintln!("✗ Connection error: {e}");
                break;
            }
            Err(_) => {} // Ignore parser errors
        }
    }
}

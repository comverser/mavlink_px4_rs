use crate::{connection::MavConn, display};
use mavlink::error::MessageReadError;
use std::{
    collections::{HashMap, HashSet},
    io::ErrorKind,
    thread,
    time::{Duration, Instant},
};

const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Main message receiver loop
pub fn run(vehicle: &MavConn, filter: Option<HashSet<String>>) {
    println!("Listening for MAVLink messages...\n");

    let mut frequency_tracker = FrequencyTracker::new();

    loop {
        match vehicle.recv() {
            Ok((header, msg)) => {
                let frequency = frequency_tracker.calculate(&msg);
                display::show(&header, &msg, &filter, frequency);
            }
            Err(e) if should_retry(&e) => {
                eprintln!("⏳ Waiting for data...");
                thread::sleep(POLL_INTERVAL);
            }
            Err(e) if is_connection_lost(&e) => {
                eprintln!("\n✗ Connection lost: {e}");
                break;
            }
            Err(e) => {
                eprintln!("✗ Message error: {e}");
            }
        }
    }
}

struct FrequencyTracker {
    last_times: HashMap<String, Instant>,
}

impl FrequencyTracker {
    fn new() -> Self {
        Self {
            last_times: HashMap::new(),
        }
    }

    fn calculate(&mut self, msg: &mavlink::ardupilotmega::MavMessage) -> Option<f64> {
        let now = Instant::now();
        let msg_type = display::extract_message_type(msg);

        let frequency = self.last_times.get(&msg_type).map(|last| {
            let interval = now.duration_since(*last).as_secs_f64();
            if interval > 0.0 {
                1.0 / interval
            } else {
                0.0
            }
        });

        self.last_times.insert(msg_type, now);
        frequency
    }
}

fn should_retry(error: &MessageReadError) -> bool {
    matches!(
        error,
        MessageReadError::Io(e) if e.kind() == ErrorKind::WouldBlock
    )
}

fn is_connection_lost(error: &MessageReadError) -> bool {
    matches!(error, MessageReadError::Io(_))
}

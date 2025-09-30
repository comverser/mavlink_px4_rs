use mavlink::error::MessageReadError;
use std::{env, sync::Arc, thread, time::Duration};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage: mavlink-dump (tcpout|tcpin|udpout|udpin|udpbcast|serial|file):(ip|dev|path):(port|baud)"
        );
        return;
    }

    println!("Connecting to: {}", &args[1]);

    // It's possible to change the mavlink dialect to be used in the connect call
    let mut mavconn = match mavlink::connect::<mavlink::ardupilotmega::MavMessage>(&args[1]) {
        Ok(conn) => {
            println!("✓ Socket opened successfully!");
            conn
        }
        Err(e) => {
            println!("✗ Failed to open connection: {}", e);
            return;
        }
    };

    // PX4 uses MAVLink V2 by default
    mavconn.set_protocol_version(mavlink::MavlinkVersion::V2);

    let vehicle = Arc::new(mavconn);

    vehicle
        .send(&mavlink::MavHeader::default(), &request_parameters())
        .unwrap();

    vehicle
        .send(&mavlink::MavHeader::default(), &request_stream())
        .unwrap();

    println!("✓ Starting heartbeat thread...");
    thread::spawn({
        let vehicle = vehicle.clone();
        move || loop {
            let res = vehicle.send_default(&heartbeat_message());
            if res.is_ok() {
                thread::sleep(Duration::from_secs(1));
            } else {
                println!("send failed: {res:?}");
            }
        }
    });

    println!("Listening for MAVLink messages...");
    let mut message_count = 0;

    loop {
        match vehicle.recv() {
            Ok((header, msg)) => {
                message_count += 1;

                // Show simplified output for common messages
                match &msg {
                    mavlink::ardupilotmega::MavMessage::HEARTBEAT(hb) => {
                        println!(
                            "[{}] HEARTBEAT from system {}, component {}: type={:?}, autopilot={:?}, status={:?}",
                            message_count,
                            header.system_id,
                            header.component_id,
                            hb.mavtype,
                            hb.autopilot,
                            hb.system_status
                        );
                    }
                    mavlink::ardupilotmega::MavMessage::ATTITUDE(att) => {
                        println!(
                            "[{}] ATTITUDE: roll={:.2}°, pitch={:.2}°, yaw={:.2}°",
                            message_count,
                            att.roll.to_degrees(),
                            att.pitch.to_degrees(),
                            att.yaw.to_degrees()
                        );
                    }
                    mavlink::ardupilotmega::MavMessage::GLOBAL_POSITION_INT(pos) => {
                        println!(
                            "[{}] GPS POSITION: lat={}, lon={}, alt={}m",
                            message_count,
                            pos.lat as f64 / 1e7,
                            pos.lon as f64 / 1e7,
                            pos.alt as f64 / 1000.0
                        );
                    }
                    mavlink::ardupilotmega::MavMessage::PARAM_VALUE(param) => {
                        let param_name = String::from_utf8_lossy(&param.param_id)
                            .trim_end_matches('\0')
                            .to_string();
                        println!(
                            "[{}] PARAM: {} = {}",
                            message_count, param_name, param.param_value
                        );
                    }
                    _ => {
                        // For other messages, show a compact format
                        let msg_string = format!("{:?}", msg);
                        let msg_name = msg_string.split('(').next().unwrap_or("UNKNOWN");
                        println!(
                            "[{}] {} from system {}, component {}",
                            message_count, msg_name, header.system_id, header.component_id
                        );
                    }
                }
            }
            Err(MessageReadError::Io(e)) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    //no messages currently available to receive -- wait a while
                    thread::sleep(Duration::from_millis(100));
                    if message_count == 0 {
                        println!(
                            "⏳ Waiting for messages from PX4... (make sure PX4 SITL is running)"
                        );
                    }
                    continue;
                } else {
                    println!("✗ Connection error: {e:?}");
                    break;
                }
            }
            // messages that didn't get through due to parser errors are ignored
            _ => {}
        }
    }
}

/// Create a heartbeat message using 'ardupilotmega' dialect
pub fn heartbeat_message() -> mavlink::ardupilotmega::MavMessage {
    mavlink::ardupilotmega::MavMessage::HEARTBEAT(mavlink::ardupilotmega::HEARTBEAT_DATA {
        custom_mode: 0,
        mavtype: mavlink::ardupilotmega::MavType::MAV_TYPE_QUADROTOR,
        autopilot: mavlink::ardupilotmega::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
        base_mode: mavlink::ardupilotmega::MavModeFlag::empty(),
        system_status: mavlink::ardupilotmega::MavState::MAV_STATE_STANDBY,
        mavlink_version: 0x3,
    })
}

/// Create a message requesting the parameters list
pub fn request_parameters() -> mavlink::ardupilotmega::MavMessage {
    mavlink::ardupilotmega::MavMessage::PARAM_REQUEST_LIST(
        mavlink::ardupilotmega::PARAM_REQUEST_LIST_DATA {
            target_system: 1,    // PX4 typically uses system ID 1
            target_component: 1, // Main component
        },
    )
}

/// Create a message enabling data streaming
pub fn request_stream() -> mavlink::ardupilotmega::MavMessage {
    #[expect(deprecated)]
    mavlink::ardupilotmega::MavMessage::REQUEST_DATA_STREAM(
        mavlink::ardupilotmega::REQUEST_DATA_STREAM_DATA {
            target_system: 1,    // PX4 system ID
            target_component: 1, // Main component
            req_stream_id: 0,    // All streams
            req_message_rate: 10,
            start_stop: 1,
        },
    )
}

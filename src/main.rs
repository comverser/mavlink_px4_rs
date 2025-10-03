use mavlink::error::MessageReadError;
use std::{env, sync::Arc, thread, time::Duration};

// ============================================================================
// Type Aliases
// ============================================================================

type MavConn = Arc<Box<dyn mavlink::MavConnection<mavlink::ardupilotmega::MavMessage> + Sync + Send>>;

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() {
    let connection_string = match parse_args() {
        Some(s) => s,
        None => return,
    };

    let vehicle = match connect_to_vehicle(&connection_string) {
        Ok(v) => v,
        Err(_) => return,
    };

    initialize_vehicle(&vehicle);
    start_heartbeat_thread(&vehicle);
    message_receive_loop(&vehicle);
}

// ============================================================================
// Command Line Argument Parsing
// ============================================================================

/// Parse command line arguments and return the connection string
fn parse_args() -> Option<String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage: mavlink-dump (tcpout|tcpin|udpout|udpin|udpbcast|serial|file):(ip|dev|path):(port|baud)"
        );
        return None;
    }

    Some(args[1].clone())
}

// ============================================================================
// Connection Management
// ============================================================================

/// Connect to the MAVLink vehicle
fn connect_to_vehicle(connection_string: &str) -> Result<MavConn, ()> {
    println!("Connecting to: {}", connection_string);

    let mut mavconn = match mavlink::connect::<mavlink::ardupilotmega::MavMessage>(connection_string)
    {
        Ok(conn) => {
            println!("✓ Socket opened successfully!");
            conn
        }
        Err(e) => {
            println!("✗ Failed to open connection: {}", e);
            return Err(());
        }
    };

    // PX4 uses MAVLink V2 by default
    mavconn.set_protocol_version(mavlink::MavlinkVersion::V2);

    Ok(Arc::new(mavconn))
}

/// Initialize the vehicle by requesting parameters and data streams
fn initialize_vehicle(vehicle: &MavConn) {
    vehicle
        .send(&mavlink::MavHeader::default(), &request_parameters())
        .unwrap();

    vehicle
        .send(&mavlink::MavHeader::default(), &request_stream())
        .unwrap();
}

// ============================================================================
// Background Tasks
// ============================================================================

/// Start a background thread that sends periodic heartbeat messages
fn start_heartbeat_thread(vehicle: &MavConn) {
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
}

// ============================================================================
// Message Reception
// ============================================================================

/// Main loop for receiving and displaying MAVLink messages
fn message_receive_loop(vehicle: &MavConn) {
    println!("Listening for MAVLink messages...");
    println!("(Press Ctrl+C if nothing appears after 10 seconds)");
    let mut received_first_message = false;

    loop {
        match vehicle.recv() {
            Ok((header, msg)) => {
                received_first_message = true;
                display_message(&header, &msg);
            }
            Err(MessageReadError::Io(e)) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    handle_no_messages(&mut received_first_message);
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

/// Handle the case when no messages are currently available
fn handle_no_messages(received_first_message: &mut bool) {
    thread::sleep(Duration::from_millis(100));
    if !*received_first_message {
        println!("⏳ Waiting for messages from PX4... (make sure PX4 SITL is running)");
    }
}

// ============================================================================
// Message Display
// ============================================================================

/// Display a received MAVLink message with appropriate formatting
fn display_message(
    header: &mavlink::MavHeader,
    msg: &mavlink::ardupilotmega::MavMessage,
) {
    match msg {
        mavlink::ardupilotmega::MavMessage::HEARTBEAT(hb) => {
            display_heartbeat(header, hb);
        }
        mavlink::ardupilotmega::MavMessage::ATTITUDE(att) => {
            display_attitude(att);
        }
        mavlink::ardupilotmega::MavMessage::GLOBAL_POSITION_INT(pos) => {
            display_gps_position(pos);
        }
        mavlink::ardupilotmega::MavMessage::PARAM_VALUE(param) => {
            display_parameter(param);
        }
        _ => {
            display_generic_message(header, msg);
        }
    }
}

/// Display a HEARTBEAT message
fn display_heartbeat(
    header: &mavlink::MavHeader,
    hb: &mavlink::ardupilotmega::HEARTBEAT_DATA,
) {
    println!(
        "HEARTBEAT from system {}, component {}: type={:?}, autopilot={:?}, status={:?}",
        header.system_id, header.component_id, hb.mavtype, hb.autopilot, hb.system_status
    );
}

/// Display an ATTITUDE message
fn display_attitude(att: &mavlink::ardupilotmega::ATTITUDE_DATA) {
    println!(
        "ATTITUDE: roll={:.2}°, pitch={:.2}°, yaw={:.2}°",
        att.roll.to_degrees(),
        att.pitch.to_degrees(),
        att.yaw.to_degrees()
    );
}

/// Display a GLOBAL_POSITION_INT message
fn display_gps_position(pos: &mavlink::ardupilotmega::GLOBAL_POSITION_INT_DATA) {
    println!(
        "GPS POSITION: lat={}, lon={}, alt={}m",
        pos.lat as f64 / 1e7,
        pos.lon as f64 / 1e7,
        pos.alt as f64 / 1000.0
    );
}

/// Display a PARAM_VALUE message
fn display_parameter(param: &mavlink::ardupilotmega::PARAM_VALUE_DATA) {
    let param_name = String::from_utf8_lossy(&param.param_id)
        .trim_end_matches('\0')
        .to_string();
    println!("PARAM: {} = {}", param_name, param.param_value);
}

/// Display a generic message in compact format
fn display_generic_message(
    header: &mavlink::MavHeader,
    msg: &mavlink::ardupilotmega::MavMessage,
) {
    let msg_string = format!("{:?}", msg);
    let msg_name = msg_string.split('(').next().unwrap_or("UNKNOWN");
    println!(
        "{} from system {}, component {}",
        msg_name, header.system_id, header.component_id
    );
}

// ============================================================================
// MAVLink Message Constructors
// ============================================================================

/// Create a heartbeat message using 'ardupilotmega' dialect
fn heartbeat_message() -> mavlink::ardupilotmega::MavMessage {
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
fn request_parameters() -> mavlink::ardupilotmega::MavMessage {
    mavlink::ardupilotmega::MavMessage::PARAM_REQUEST_LIST(
        mavlink::ardupilotmega::PARAM_REQUEST_LIST_DATA {
            target_system: 1,    // PX4 typically uses system ID 1
            target_component: 1, // Main component
        },
    )
}

/// Create a message enabling data streaming
fn request_stream() -> mavlink::ardupilotmega::MavMessage {
    #[allow(deprecated)]
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

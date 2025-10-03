use mavlink::{
    ardupilotmega::{
        MavMessage, ATTITUDE_DATA, GLOBAL_POSITION_INT_DATA, HEARTBEAT_DATA, PARAM_VALUE_DATA,
    },
    MavHeader,
};
use std::collections::HashSet;

/// Display a MAVLink message with optional filtering
pub fn show(header: &MavHeader, msg: &MavMessage, filter: &Option<HashSet<String>>) {
    if !should_display(msg, filter) {
        return;
    }

    match msg {
        MavMessage::HEARTBEAT(data) => print_heartbeat(header, data),
        MavMessage::ATTITUDE(data) => print_attitude(data),
        MavMessage::GLOBAL_POSITION_INT(data) => print_position(data),
        MavMessage::PARAM_VALUE(data) => print_parameter(data),
        _ => print_generic(header, msg),
    }
}

fn should_display(msg: &MavMessage, filter: &Option<HashSet<String>>) -> bool {
    let Some(allowed_messages) = filter else {
        return true;
    };

    let msg_type = extract_message_type(msg);
    allowed_messages.contains(&msg_type)
}

fn extract_message_type(msg: &MavMessage) -> String {
    format!("{msg:?}")
        .split('(')
        .next()
        .unwrap_or("UNKNOWN")
        .to_string()
}

// Message formatters
fn print_heartbeat(header: &MavHeader, data: &HEARTBEAT_DATA) {
    println!(
        "HEARTBEAT [{}/{}] type={:?}, autopilot={:?}, state={:?}",
        header.system_id, header.component_id, data.mavtype, data.autopilot, data.system_status
    );
}

fn print_attitude(data: &ATTITUDE_DATA) {
    println!(
        "ATTITUDE  roll={:>6.2}° pitch={:>6.2}° yaw={:>6.2}°",
        data.roll.to_degrees(),
        data.pitch.to_degrees(),
        data.yaw.to_degrees()
    );
}

fn print_position(data: &GLOBAL_POSITION_INT_DATA) {
    println!(
        "POSITION  lat={:>10.6} lon={:>10.6} alt={:>7.1}m",
        data.lat as f64 / 1e7,
        data.lon as f64 / 1e7,
        data.alt as f64 / 1000.0
    );
}

fn print_parameter(data: &PARAM_VALUE_DATA) {
    let name = String::from_utf8_lossy(&data.param_id)
        .trim_end_matches('\0')
        .to_string();
    println!("PARAM     {name} = {}", data.param_value);
}

fn print_generic(header: &MavHeader, msg: &MavMessage) {
    let msg_type = extract_message_type(msg);
    println!("{msg_type} [{}/{}]", header.system_id, header.component_id);
}

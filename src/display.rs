use mavlink::{
    ardupilotmega::{
        MavMessage, ATTITUDE_DATA, GLOBAL_POSITION_INT_DATA, HEARTBEAT_DATA, PARAM_VALUE_DATA,
    },
    MavHeader,
};

pub fn show(header: &MavHeader, msg: &MavMessage) {
    match msg {
        MavMessage::HEARTBEAT(data) => heartbeat(header, data),
        MavMessage::ATTITUDE(data) => attitude(data),
        MavMessage::GLOBAL_POSITION_INT(data) => position(data),
        MavMessage::PARAM_VALUE(data) => parameter(data),
        _ => other(header, msg),
    }
}

fn heartbeat(header: &MavHeader, data: &HEARTBEAT_DATA) {
    println!(
        "HEARTBEAT [{}/{}] type={:?}, autopilot={:?}, state={:?}",
        header.system_id, header.component_id, data.mavtype, data.autopilot, data.system_status
    );
}

fn attitude(data: &ATTITUDE_DATA) {
    println!(
        "ATTITUDE  roll={:>6.2}° pitch={:>6.2}° yaw={:>6.2}°",
        data.roll.to_degrees(),
        data.pitch.to_degrees(),
        data.yaw.to_degrees()
    );
}

fn position(data: &GLOBAL_POSITION_INT_DATA) {
    println!(
        "POSITION  lat={:>10.6} lon={:>10.6} alt={:>7.1}m",
        data.lat as f64 / 1e7,
        data.lon as f64 / 1e7,
        data.alt as f64 / 1000.0
    );
}

fn parameter(data: &PARAM_VALUE_DATA) {
    let name = String::from_utf8_lossy(&data.param_id)
        .trim_end_matches('\0')
        .to_string();
    println!("PARAM     {name} = {}", data.param_value);
}

fn other(header: &MavHeader, msg: &MavMessage) {
    let msg_type = format!("{msg:?}")
        .split('(')
        .next()
        .unwrap_or("UNKNOWN")
        .to_string();
    println!("{msg_type} [{}/{}]", header.system_id, header.component_id);
}

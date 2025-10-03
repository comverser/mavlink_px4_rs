use crate::connection::MavConn;
use mavlink::ardupilotmega::{
    MavAutopilot, MavMessage, MavModeFlag, MavState, MavType, HEARTBEAT_DATA,
    PARAM_REQUEST_LIST_DATA, REQUEST_DATA_STREAM_DATA,
};
use mavlink::MavHeader;
use std::{thread, time::Duration};

const TARGET_SYSTEM_ID: u8 = 1;
const TARGET_COMPONENT_ID: u8 = 1;
const STREAM_RATE_HZ: u16 = 10;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);

pub fn initialize(vehicle: &MavConn) {
    send_message(vehicle, &request_parameters());
    send_message(vehicle, &request_stream());
}

pub fn start_heartbeat(vehicle: &MavConn) {
    let vehicle = vehicle.clone();
    thread::spawn(move || loop {
        vehicle.send_default(&heartbeat()).ok();
        thread::sleep(HEARTBEAT_INTERVAL);
    });
}

fn send_message(vehicle: &MavConn, msg: &MavMessage) {
    if let Err(e) = vehicle.send(&MavHeader::default(), msg) {
        eprintln!("Warning: Failed to send message: {e}");
    }
}

fn heartbeat() -> MavMessage {
    MavMessage::HEARTBEAT(HEARTBEAT_DATA {
        custom_mode: 0,
        mavtype: MavType::MAV_TYPE_QUADROTOR,
        autopilot: MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
        base_mode: MavModeFlag::empty(),
        system_status: MavState::MAV_STATE_STANDBY,
        mavlink_version: 0x3,
    })
}

fn request_parameters() -> MavMessage {
    MavMessage::PARAM_REQUEST_LIST(PARAM_REQUEST_LIST_DATA {
        target_system: TARGET_SYSTEM_ID,
        target_component: TARGET_COMPONENT_ID,
    })
}

fn request_stream() -> MavMessage {
    #[allow(deprecated)]
    MavMessage::REQUEST_DATA_STREAM(REQUEST_DATA_STREAM_DATA {
        target_system: TARGET_SYSTEM_ID,
        target_component: TARGET_COMPONENT_ID,
        req_stream_id: 0,
        req_message_rate: STREAM_RATE_HZ,
        start_stop: 1,
    })
}

use crate::connection::MavConn;
use mavlink::{
    ardupilotmega::{
        MavAutopilot, MavMessage, MavModeFlag, MavState, MavType, HEARTBEAT_DATA,
        PARAM_REQUEST_LIST_DATA, REQUEST_DATA_STREAM_DATA,
    },
    MavHeader,
};
use std::{thread, time::Duration};

const TARGET_SYSTEM: u8 = 1;
const TARGET_COMPONENT: u8 = 1;
const STREAM_RATE_HZ: u16 = 10;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);

pub fn initialize(vehicle: &MavConn) {
    send(vehicle, &request_parameters());
    send(vehicle, &request_stream());
}

pub fn start_heartbeat(vehicle: &MavConn) {
    let vehicle = vehicle.clone();
    thread::spawn(move || loop {
        if let Err(e) = vehicle.send_default(&heartbeat()) {
            eprintln!("Heartbeat failed: {e:?}");
        }
        thread::sleep(HEARTBEAT_INTERVAL);
    });
}

fn send(vehicle: &MavConn, msg: &MavMessage) {
    if let Err(e) = vehicle.send(&MavHeader::default(), msg) {
        eprintln!("Send failed: {e}");
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
        target_system: TARGET_SYSTEM,
        target_component: TARGET_COMPONENT,
    })
}

fn request_stream() -> MavMessage {
    #[allow(deprecated)]
    MavMessage::REQUEST_DATA_STREAM(REQUEST_DATA_STREAM_DATA {
        target_system: TARGET_SYSTEM,
        target_component: TARGET_COMPONENT,
        req_stream_id: 0,
        req_message_rate: STREAM_RATE_HZ,
        start_stop: 1,
    })
}

use super::*;

macro_rules! mavstate {
    ($from:path => $to:path) => {
        paste! {
            pub fn [<$from _to_ $to>](mavstate: $from::MavState) -> $to::MavState {
                match mavstate {
                    $from::MavState::MAV_STATE_UNINIT => $to::MavState::MAV_STATE_UNINIT,
                    $from::MavState::MAV_STATE_BOOT => $to::MavState::MAV_STATE_BOOT,
                    $from::MavState::MAV_STATE_CALIBRATING => $to::MavState::MAV_STATE_CALIBRATING,
                    $from::MavState::MAV_STATE_STANDBY => $to::MavState::MAV_STATE_STANDBY,
                    $from::MavState::MAV_STATE_ACTIVE => $to::MavState::MAV_STATE_ACTIVE,
                    $from::MavState::MAV_STATE_CRITICAL => $to::MavState::MAV_STATE_CRITICAL,
                    $from::MavState::MAV_STATE_EMERGENCY => $to::MavState::MAV_STATE_EMERGENCY,
                    $from::MavState::MAV_STATE_POWEROFF => $to::MavState::MAV_STATE_POWEROFF,
                    $from::MavState::MAV_STATE_FLIGHT_TERMINATION => $to::MavState::MAV_STATE_FLIGHT_TERMINATION,
                }
            }
        }
    };
}

mavstate!(common => minimal);
mavstate!(minimal => common);

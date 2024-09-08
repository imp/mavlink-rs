use super::*;

macro_rules! mavautopilot {
    ($from:path => $to:path) => {
        paste! {
            pub fn [<$from _to_ $to>](mavautopilot: $from::MavAutopilot) -> $to::MavAutopilot {
                match mavautopilot {
                    $from::MavAutopilot::MAV_AUTOPILOT_GENERIC => $to::MavAutopilot::MAV_AUTOPILOT_GENERIC,
                    $from::MavAutopilot::MAV_AUTOPILOT_RESERVED => $to::MavAutopilot::MAV_AUTOPILOT_RESERVED,
                    $from::MavAutopilot::MAV_AUTOPILOT_SLUGS => $to::MavAutopilot::MAV_AUTOPILOT_SLUGS,
                    $from::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA => $to::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
                    $from::MavAutopilot::MAV_AUTOPILOT_OPENPILOT => $to::MavAutopilot::MAV_AUTOPILOT_OPENPILOT,
                    $from::MavAutopilot::MAV_AUTOPILOT_GENERIC_WAYPOINTS_ONLY => $to::MavAutopilot::MAV_AUTOPILOT_GENERIC_WAYPOINTS_ONLY,
                    $from::MavAutopilot::MAV_AUTOPILOT_GENERIC_WAYPOINTS_AND_SIMPLE_NAVIGATION_ONLY => $to::MavAutopilot::MAV_AUTOPILOT_GENERIC_WAYPOINTS_AND_SIMPLE_NAVIGATION_ONLY,
                    $from::MavAutopilot::MAV_AUTOPILOT_GENERIC_MISSION_FULL => $to::MavAutopilot::MAV_AUTOPILOT_GENERIC_MISSION_FULL,
                    $from::MavAutopilot::MAV_AUTOPILOT_INVALID => $to::MavAutopilot::MAV_AUTOPILOT_INVALID,
                    $from::MavAutopilot::MAV_AUTOPILOT_PPZ => $to::MavAutopilot::MAV_AUTOPILOT_PPZ,
                    $from::MavAutopilot::MAV_AUTOPILOT_UDB => $to::MavAutopilot::MAV_AUTOPILOT_UDB,
                    $from::MavAutopilot::MAV_AUTOPILOT_FP => $to::MavAutopilot::MAV_AUTOPILOT_FP,
                    $from::MavAutopilot::MAV_AUTOPILOT_PX4 => $to::MavAutopilot::MAV_AUTOPILOT_PX4,
                    $from::MavAutopilot::MAV_AUTOPILOT_SMACCMPILOT => $to::MavAutopilot::MAV_AUTOPILOT_SMACCMPILOT,
                    $from::MavAutopilot::MAV_AUTOPILOT_AUTOQUAD => $to::MavAutopilot::MAV_AUTOPILOT_AUTOQUAD,
                    $from::MavAutopilot::MAV_AUTOPILOT_ARMAZILA => $to::MavAutopilot::MAV_AUTOPILOT_ARMAZILA,
                    $from::MavAutopilot::MAV_AUTOPILOT_AEROB => $to::MavAutopilot::MAV_AUTOPILOT_AEROB,
                    $from::MavAutopilot::MAV_AUTOPILOT_ASLUAV => $to::MavAutopilot::MAV_AUTOPILOT_ASLUAV,
                    $from::MavAutopilot::MAV_AUTOPILOT_SMARTAP => $to::MavAutopilot::MAV_AUTOPILOT_SMARTAP,
                    $from::MavAutopilot::MAV_AUTOPILOT_AIRRAILS => $to::MavAutopilot::MAV_AUTOPILOT_AIRRAILS,
                    $from::MavAutopilot::MAV_AUTOPILOT_REFLEX => $to::MavAutopilot::MAV_AUTOPILOT_REFLEX,
                }
            }
        }
    };
}

mavautopilot!(common => minimal);
mavautopilot!(minimal => common);

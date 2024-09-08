use super::*;

macro_rules! heartbeat_data {
    ($from:path => $to:path) => {
        paste! {
            pub fn [<$from _to_ $to>](data: $from::HEARTBEAT_DATA) -> $to::HEARTBEAT_DATA {
                $to::HEARTBEAT_DATA {
                    custom_mode: data.custom_mode,
                    mavtype: mavtype::[<$from _to_ $to>](data.mavtype),
                    autopilot: mavautopilot::[<$from _to_ $to>](data.autopilot),
                    base_mode: mavmodeflag::[<$from _to_ $to>](data.base_mode),
                    system_status: mavstate::[<$from _to_ $to>](data.system_status),
                    mavlink_version: data.mavlink_version,
                }
            }
        }
    };
}

heartbeat_data!(common => minimal);
heartbeat_data!(minimal => common);

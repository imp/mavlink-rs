use super::*;

macro_rules! mavtype {
    ($from:path => $to:path) => {
        paste! {
            pub fn [<$from _to_ $to>](mavtype: $from::MavType) -> $to::MavType {
                match mavtype {
                    $from::MavType::MAV_TYPE_GENERIC => $to::MavType::MAV_TYPE_GENERIC,
                    $from::MavType::MAV_TYPE_FIXED_WING => $to::MavType::MAV_TYPE_FIXED_WING,
                    $from::MavType::MAV_TYPE_QUADROTOR => $to::MavType::MAV_TYPE_QUADROTOR,
                    $from::MavType::MAV_TYPE_COAXIAL => $to::MavType::MAV_TYPE_COAXIAL,
                    $from::MavType::MAV_TYPE_HELICOPTER => $to::MavType::MAV_TYPE_HELICOPTER,
                    $from::MavType::MAV_TYPE_ANTENNA_TRACKER => $to::MavType::MAV_TYPE_ANTENNA_TRACKER,
                    $from::MavType::MAV_TYPE_GCS => $to::MavType::MAV_TYPE_GCS,
                    $from::MavType::MAV_TYPE_AIRSHIP => $to::MavType::MAV_TYPE_AIRSHIP,
                    $from::MavType::MAV_TYPE_FREE_BALLOON => $to::MavType::MAV_TYPE_FREE_BALLOON,
                    $from::MavType::MAV_TYPE_ROCKET => $to::MavType::MAV_TYPE_ROCKET,
                    $from::MavType::MAV_TYPE_GROUND_ROVER => $to::MavType::MAV_TYPE_GROUND_ROVER,
                    $from::MavType::MAV_TYPE_SURFACE_BOAT => $to::MavType::MAV_TYPE_SURFACE_BOAT,
                    $from::MavType::MAV_TYPE_SUBMARINE => $to::MavType::MAV_TYPE_SUBMARINE,
                    $from::MavType::MAV_TYPE_HEXAROTOR => $to::MavType::MAV_TYPE_HEXAROTOR,
                    $from::MavType::MAV_TYPE_OCTOROTOR => $to::MavType::MAV_TYPE_OCTOROTOR,
                    $from::MavType::MAV_TYPE_TRICOPTER => $to::MavType::MAV_TYPE_TRICOPTER,
                    $from::MavType::MAV_TYPE_FLAPPING_WING => $to::MavType::MAV_TYPE_FLAPPING_WING,
                    $from::MavType::MAV_TYPE_KITE => $to::MavType::MAV_TYPE_KITE,
                    $from::MavType::MAV_TYPE_ONBOARD_CONTROLLER => {
                        $to::MavType::MAV_TYPE_ONBOARD_CONTROLLER
                    }
                    $from::MavType::MAV_TYPE_VTOL_TAILSITTER_DUOROTOR => {
                        $to::MavType::MAV_TYPE_VTOL_TAILSITTER_DUOROTOR
                    }
                    $from::MavType::MAV_TYPE_VTOL_TAILSITTER_QUADROTOR => {
                        $to::MavType::MAV_TYPE_VTOL_TAILSITTER_QUADROTOR
                    }
                    $from::MavType::MAV_TYPE_VTOL_TILTROTOR => $to::MavType::MAV_TYPE_VTOL_TILTROTOR,
                    $from::MavType::MAV_TYPE_VTOL_FIXEDROTOR => $to::MavType::MAV_TYPE_VTOL_FIXEDROTOR,
                    $from::MavType::MAV_TYPE_VTOL_TAILSITTER => $to::MavType::MAV_TYPE_VTOL_TAILSITTER,
                    $from::MavType::MAV_TYPE_VTOL_TILTWING => $to::MavType::MAV_TYPE_VTOL_TILTWING,
                    $from::MavType::MAV_TYPE_VTOL_RESERVED5 => $to::MavType::MAV_TYPE_VTOL_RESERVED5,
                    $from::MavType::MAV_TYPE_GIMBAL => $to::MavType::MAV_TYPE_GIMBAL,
                    $from::MavType::MAV_TYPE_ADSB => $to::MavType::MAV_TYPE_ADSB,
                    $from::MavType::MAV_TYPE_PARAFOIL => $to::MavType::MAV_TYPE_PARAFOIL,
                    $from::MavType::MAV_TYPE_DODECAROTOR => $to::MavType::MAV_TYPE_DODECAROTOR,
                    $from::MavType::MAV_TYPE_CAMERA => $to::MavType::MAV_TYPE_CAMERA,
                    $from::MavType::MAV_TYPE_CHARGING_STATION => {
                        $to::MavType::MAV_TYPE_CHARGING_STATION
                    }
                    $from::MavType::MAV_TYPE_FLARM => $to::MavType::MAV_TYPE_FLARM,
                    $from::MavType::MAV_TYPE_SERVO => $to::MavType::MAV_TYPE_SERVO,
                    $from::MavType::MAV_TYPE_ODID => $to::MavType::MAV_TYPE_ODID,
                    $from::MavType::MAV_TYPE_DECAROTOR => $to::MavType::MAV_TYPE_DECAROTOR,
                    $from::MavType::MAV_TYPE_BATTERY => $to::MavType::MAV_TYPE_BATTERY,
                    $from::MavType::MAV_TYPE_PARACHUTE => $to::MavType::MAV_TYPE_PARACHUTE,
                    $from::MavType::MAV_TYPE_LOG => $to::MavType::MAV_TYPE_LOG,
                    $from::MavType::MAV_TYPE_OSD => $to::MavType::MAV_TYPE_OSD,
                    $from::MavType::MAV_TYPE_IMU => $to::MavType::MAV_TYPE_IMU,
                    $from::MavType::MAV_TYPE_GPS => $to::MavType::MAV_TYPE_GPS,
                    $from::MavType::MAV_TYPE_WINCH => $to::MavType::MAV_TYPE_WINCH,
                }
            }
        }
    };
}

mavtype!(common => minimal);
mavtype!(minimal => common);

mavtype!(minimal => ardupilotmega);
mavtype!(ardupilotmega => minimal);

mavtype!(common => ardupilotmega);
mavtype!(ardupilotmega => common);

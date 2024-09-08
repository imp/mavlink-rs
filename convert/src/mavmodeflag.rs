use super::*;

macro_rules! mavmodeflag {
    ($from:path => $to:path) => {
        paste! {
            pub fn [<$from _to_ $to>](mavmodeflag: $from::MavModeFlag) -> $to::MavModeFlag {
                $to::MavModeFlag::from_bits(mavmodeflag.bits()).unwrap_or($to::MavModeFlag::DEFAULT)
            }
        }
    };
}

mavmodeflag!(common => minimal);
mavmodeflag!(minimal => common);

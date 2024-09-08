use mavlink::common::HEARTBEAT_DATA;

use super::*;

pub trait Heartbeat {
    fn yes(&self) -> HEARTBEAT_DATA;
}

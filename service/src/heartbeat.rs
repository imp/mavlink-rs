use mavlink::common::HEARTBEAT_DATA;

pub trait Heartbeat {
    fn yes(&self) -> HEARTBEAT_DATA;
}

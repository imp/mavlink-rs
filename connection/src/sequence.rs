use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

#[derive(Debug, Default)]
pub struct Sequence(AtomicU8);

impl Sequence {
    pub fn get(&self) -> u8 {
        self.0.fetch_add(1, Ordering::SeqCst)
    }
}

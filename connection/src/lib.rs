use std::io;

use mavlink::error;
use mavlink::MavConnection;
use mavlink::MavHeader;
use mavlink::MavlinkVersion;
use mavlink::Message;

pub use diagnostics::Diagnostics;
pub use sequence::Sequence;

mod diagnostics;
mod sequence;

fn default<T: Default>() -> T {
    T::default()
}

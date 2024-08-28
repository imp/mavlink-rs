use parking_lot::Mutex;

use super::*;

#[derive(Debug)]
pub struct Diagnostics {
    protocol_version: MavlinkVersion,
    sequence: Sequence,
    messages: Mutex<Vec<Vec<u8>>>,
}

impl Diagnostics {
    pub fn v2() -> Self {
        Self {
            protocol_version: MavlinkVersion::V2,
            sequence: default(),
            messages: default(),
        }
    }
}

impl Diagnostics {
    fn recv_impl<M>(&self) -> Result<(MavHeader, M), error::MessageReadError>
    where
        M: Message,
    {
        let buf = Vec::with_capacity(280);
        let buf = io::Cursor::new(buf);
        let mut buf = PeekReader::new(buf);
        mavlink::read_versioned_msg(&mut buf, self.protocol_version)
    }

    fn send_impl<M>(&self, header: &MavHeader, data: &M) -> Result<usize, error::MessageWriteError>
    where
        M: Message,
    {
        let header = MavHeader {
            sequence: self.sequence.get(),
            ..*header
        };
        let mut buf = Vec::with_capacity(280);
        let count = mavlink::write_versioned_msg(&mut buf, self.protocol_version, header, data)?;
        buf.shrink_to(count);
        self.put(buf);
        Ok(count)
    }

    fn put(&self, buf: Vec<u8>) {
        self.messages.lock().push(buf)
    }
}

impl<M> MavConnection<M> for Diagnostics
where
    M: Message,
{
    fn recv(&self) -> Result<(MavHeader, M), error::MessageReadError> {
        self.recv_impl()
    }

    fn send(&self, header: &MavHeader, data: &M) -> Result<usize, error::MessageWriteError> {
        self.send_impl(header, data)
    }

    fn set_protocol_version(&mut self, version: MavlinkVersion) {
        assert_eq!(version, MavlinkVersion::V2);
    }

    fn get_protocol_version(&self) -> MavlinkVersion {
        self.protocol_version
    }
}

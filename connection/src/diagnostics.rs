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
    fn recv<M>(&self) -> io::Result<(MavHeader, M)>
    where
        M: Message,
    {
        Err(io::Error::other("Diagnostics recv is not implemented yet"))
    }
}

impl Diagnostics {
    fn put(&self, buf: Vec<u8>) {
        self.messages.lock().push(buf)
    }
}

impl<M> MavConnection<M> for Diagnostics
where
    M: Message,
{
    fn recv(&self) -> Result<(MavHeader, M), error::MessageReadError> {
        let (header, message) = self.recv()?;
        Ok((header, message))
    }

    fn send(&self, header: &MavHeader, data: &M) -> Result<usize, error::MessageWriteError> {
        let header = MavHeader {
            system_id: header.system_id,
            component_id: header.component_id,
            sequence: self.sequence.get(),
        };

        let mut buf = Vec::with_capacity(280);
        let count = mavlink::write_versioned_msg(&mut buf, self.protocol_version, header, data)?;
        self.put(buf);
        Ok(count)
    }

    fn set_protocol_version(&mut self, version: MavlinkVersion) {
        assert_eq!(version, MavlinkVersion::V2);
    }

    fn get_protocol_version(&self) -> MavlinkVersion {
        self.protocol_version
    }
}

use crate::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

/// Acknowledgement to pubrel
#[derive(Debug, Clone, PartialEq)]
pub struct PubComp {
    pub pkid: u16,
}

impl PubComp {
    pub fn new(pkid: u16) -> PubComp {
        PubComp { pkid }
    }

    pub(crate) fn assemble(fixed_header: FixedHeader, mut bytes: Bytes) -> Result<Self, Error> {
        if fixed_header.remaining_len != 2 {
            return Err(Error::PayloadSizeIncorrect);
        }

        let variable_header_index = fixed_header.fixed_header_len;
        bytes.advance(variable_header_index);
        let pkid = read_u16(&mut bytes)?;
        let pubcomp = PubComp { pkid };

        Ok(pubcomp)
    }

    pub fn write(&self, buffer: &mut BytesMut) -> Result<usize, Error> {
        let o: &[u8] = &[0x70, 0x02];
        buffer.put_slice(o);
        buffer.put_u16(self.pkid);
        Ok(4)
    }
}

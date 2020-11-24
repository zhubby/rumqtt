use crate::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

/// Acknowledgement to QoS1 publish
#[derive(Debug, Clone, PartialEq)]
pub struct PubAck {
    pub pkid: u16,
}

impl PubAck {
    pub fn new(pkid: u16) -> PubAck {
        PubAck { pkid }
    }

    pub(crate) fn assemble(fixed_header: FixedHeader, mut bytes: Bytes) -> Result<Self, Error> {
        if fixed_header.remaining_len != 2 {
            return Err(Error::PayloadSizeIncorrect);
        }

        let variable_header_index = fixed_header.fixed_header_len;
        bytes.advance(variable_header_index);
        let pkid = bytes.get_u16();
        let puback = PubAck { pkid };

        Ok(puback)
    }

    pub fn write(&self, buffer: &mut BytesMut) -> Result<usize, Error> {
        let o: &[u8] = &[0x40, 0x02];
        buffer.put_slice(o);
        buffer.put_u16(self.pkid);
        Ok(4)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bytes::BytesMut;
    use pretty_assertions::assert_eq;

    #[test]
    fn puback_stitching_works_correctly() {
        let stream = &[
            0b0100_0000,
            0x02, // packet type, flags and remaining len
            0x00,
            0x0A, // fixed header. packet identifier = 10
            0xDE,
            0xAD,
            0xBE,
            0xEF, // extra packets in the stream
        ];
        let mut stream = BytesMut::from(&stream[..]);
        let fixed_header = parse_fixed_header(stream.iter()).unwrap();
        let ack_bytes = stream.split_to(fixed_header.frame_length()).freeze();
        let packet = PubAck::assemble(fixed_header, ack_bytes).unwrap();


        assert_eq!(packet, PubAck { pkid: 10 });
    }
}

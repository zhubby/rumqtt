pub mod packets;

use crate::*;
pub use packets::*;
use bytes::BytesMut;

/// Reads a stream of bytes and extracts MQTT packets
pub fn mqtt_read(stream: &mut BytesMut, max_packet_size: usize) -> Result<Packet, Error> {
    let fixed_header = check(stream.iter(), max_packet_size)?;

    // Test with a stream with exactly the size to check border panics
    let packet = stream.split_to(fixed_header.frame_length());
    let packet_type = fixed_header.packet_type()?;

    if fixed_header.remaining_len == 0 {
        // no payload packets
        return match packet_type {
            PacketType::PingReq => Ok(Packet::PingReq),
            PacketType::PingResp => Ok(Packet::PingResp),
            PacketType::Disconnect => Ok(Packet::Disconnect),
            _ => Err(Error::PayloadRequired),
        };
    }

    let packet = packet.freeze();
    let packet = match packet_type {
        PacketType::Connect => Packet::Connect(Connect::assemble(fixed_header, packet)?),
        PacketType::ConnAck => Packet::ConnAck(ConnAck::assemble(fixed_header, packet)?),
        PacketType::Publish => Packet::Publish(Publish::assemble(fixed_header, packet)?),
        PacketType::PubAck => Packet::PubAck(PubAck::assemble(fixed_header, packet)?),
        PacketType::PubRec => Packet::PubRec(PubRec::assemble(fixed_header, packet)?),
        PacketType::PubRel => Packet::PubRel(PubRel::assemble(fixed_header, packet)?),
        PacketType::PubComp => Packet::PubComp(PubComp::assemble(fixed_header, packet)?),
        PacketType::Subscribe => Packet::Subscribe(Subscribe::assemble(fixed_header, packet)?),
        PacketType::SubAck => Packet::SubAck(SubAck::assemble(fixed_header, packet)?),
        PacketType::Unsubscribe => {
            Packet::Unsubscribe(Unsubscribe::assemble(fixed_header, packet)?)
        }
        PacketType::UnsubAck => Packet::UnsubAck(UnsubAck::assemble(fixed_header, packet)?),
        PacketType::PingReq => Packet::PingReq,
        PacketType::PingResp => Packet::PingResp,
        PacketType::Disconnect => Packet::Disconnect,
    };

    Ok(packet)
}


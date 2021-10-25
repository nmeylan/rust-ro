use packets::packets::Packet;

pub fn chain_packets(packets: Vec<&dyn Packet>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for packet in packets {
        res = res.iter().cloned().chain(packet.raw().iter().cloned()).collect();
    }
    res
}
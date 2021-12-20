use packets::packets::Packet;

pub fn chain_packets(packets: Vec<&dyn Packet>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for packet in packets {
        res = res.iter().cloned().chain(packet.raw().iter().cloned()).collect();
    }
    res
}
pub fn chain_packets_raws(packets_raw: Vec<&Vec<u8>>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for raw in packets_raw {
        res = res.iter().cloned().chain(raw.iter().cloned()).collect();
    }
    res
}
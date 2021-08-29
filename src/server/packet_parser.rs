use crate::server::packet_db::PACKETS_DB;

    pub fn packet_name(packet: &[u8]) -> String {
        let packet_id = format!("{:02X?}{:02X?}", packet[1], packet[0]);
        match PACKETS_DB.get(packet_id.as_str()) {
            Some(v) => v.into(),
            None => format!("Unknown Packet {}", packet_id).to_string()
        }
    }
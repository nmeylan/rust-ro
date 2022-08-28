pub enum Notification {
    Char(CharNotification),
    Area(AreaNotification),
}

pub struct CharNotification {
    account_id: u32,
    packet: Vec<u8>,
}

impl CharNotification {
    pub fn new(account_id: u32, packet: Vec<u8>) -> Self {
        Self {
            account_id,
            packet
        }
    }
    pub fn account_id(&self) -> u32 {
        self.account_id
    }

    pub fn serialized_packet(&self) -> &Vec<u8> {
        &self.packet
    }
}

pub struct AreaNotification {
    map_name: String,
    map_instance_id: usize,
    range_type: AreaNotificationRangeType,
    packet: Vec<u8>,
}

pub enum AreaNotificationRangeType {
    Map, // Notify all players of the map,
    Fov {x: u16, y: u16} // Notify all players in the FoV of the point.
}
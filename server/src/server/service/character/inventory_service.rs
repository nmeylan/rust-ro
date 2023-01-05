use std::sync::{Arc, Once};
use rand::RngCore;
use tokio::runtime::Runtime;
use packets::packets::{EquipmentitemExtrainfo301, EQUIPSLOTINFO, NormalitemExtrainfo3, Packet, PacketZcEquipmentItemlist3, PacketZcItemPickupAck3, PacketZcNormalItemlist3, PacketZcPcPurchaseResult, PacketZcReqTakeoffEquipAck, PacketZcReqTakeoffEquipAck2, PacketZcReqWearEquipAck, PacketZcReqWearEquipAck2};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::events::game_event::{CharacterAddItems, CharacterEquipItem, CharacterZeny};
use crate::server::events::game_event::GameEvent::{CharacterUpdateWeight, CharacterUpdateZeny};
use crate::server::events::persistence_event::InventoryItemUpdate;
use crate::server::Server;
use crate::server::state::character::Character;
use crate::util::packet::{chain_packets, chain_packets_raws_by_value};

static mut SERVICE_INSTANCE: Option<InventoryService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct InventoryService {}

impl InventoryService {
    pub fn instance() -> &'static InventoryService {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(InventoryService::new());
        });
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    fn new() -> Self {
        Self {}
    }

    pub fn add_items_in_inventory(&self, server_ref: &Server, runtime: &Runtime, add_items: CharacterAddItems, character: &mut Character) {
        runtime.block_on(async {
            let mut rng = rand::thread_rng();
            let inventory_item_updates: Vec<InventoryItemUpdate> = add_items.items.iter().map(|item| {
                if item.item_type.is_stackable() {
                    InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id as i32, amount: item.amount as i16, stackable: true, identified: item.is_identified, unique_id: 0 }
                } else {
                    InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id as i32, amount: item.amount as i16, stackable: false, identified: item.is_identified, unique_id: rng.next_u32() as i64 }
                }
            }).collect();
            let result = server_ref.repository.character_inventory_update(&inventory_item_updates, add_items.buy).await;
            if result.is_ok() {
                let mut packets = vec![];
                character.add_items(add_items.items).iter().for_each(|(index, item)| {
                    let mut packet_zc_item_pickup_ack3 = PacketZcItemPickupAck3::new();
                    packet_zc_item_pickup_ack3.set_itid(item.item_id as u16);
                    packet_zc_item_pickup_ack3.set_count(item.amount as u16);
                    packet_zc_item_pickup_ack3.set_index(*index as u16);
                    packet_zc_item_pickup_ack3.set_is_identified(item.is_identified);
                    packet_zc_item_pickup_ack3.set_atype(item.item_type.value() as u8);
                    packet_zc_item_pickup_ack3.set_location(item.location as u16);
                    packet_zc_item_pickup_ack3.fill_raw();
                    packet_zc_item_pickup_ack3.pretty_debug();
                    packets.push(packet_zc_item_pickup_ack3)
                });
                let mut packets_raws_by_value = chain_packets_raws_by_value(packets.iter().map(|packet| packet.raw.clone()).collect());
                if add_items.buy {
                    let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new();
                    packet_zc_pc_purchase_result.set_result(0);
                    packet_zc_pc_purchase_result.fill_raw();
                    packets_raws_by_value.extend(packet_zc_pc_purchase_result.raw);
                    server_ref.add_to_next_tick(CharacterUpdateZeny(CharacterZeny { char_id: character.char_id, zeny: None }));
                }
                server_ref.add_to_next_tick(CharacterUpdateWeight(character.char_id));
                server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets_raws_by_value))).expect("Fail to send client notification");
            } else {
                if add_items.buy {
                    let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new();
                    packet_zc_pc_purchase_result.set_result(1);
                    packet_zc_pc_purchase_result.fill_raw();
                    server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_pc_purchase_result.raw))).expect("Fail to send client notification");
                }
                error!("{:?}", result.err());
            }
        });
    }

    pub fn reload_inventory(&self, server_ref: &Server, runtime: &Runtime, char_id: u32, character: &mut Character) {
        character.inventory = vec![];
        runtime.block_on(async {
            let items = server_ref.repository.character_inventory_fetch(char_id as i32).await.unwrap();
            character.add_items(items);
        });
        //PacketZcNormalItemlist3
        let mut packet_zc_equipment_itemlist3 = PacketZcEquipmentItemlist3::new();
        let mut equipments = vec![];
        character.inventory_equip().iter().for_each(|(index, item)| {
            let mut equipmentitem_extrainfo301 = EquipmentitemExtrainfo301::new();
            equipmentitem_extrainfo301.set_itid(item.item_id as u16);
            equipmentitem_extrainfo301.set_atype(item.item_type.value() as u8);
            equipmentitem_extrainfo301.set_index(*index as i16);
            equipmentitem_extrainfo301.set_is_damaged(item.is_damaged);
            equipmentitem_extrainfo301.set_is_identified(item.is_identified);
            equipmentitem_extrainfo301.set_location(item.location as u16);
            equipmentitem_extrainfo301.set_wear_state(item.equip as u16);
            equipmentitem_extrainfo301.set_refining_level(item.refine as u8);
            let mut equipslotinfo = EQUIPSLOTINFO::new();
            equipslotinfo.set_card1(item.card0 as u16);
            equipslotinfo.set_card2(item.card1 as u16);
            equipslotinfo.set_card3(item.card2 as u16);
            equipslotinfo.set_card4(item.card3 as u16);
            equipmentitem_extrainfo301.set_slot(equipslotinfo);
            equipmentitem_extrainfo301.fill_raw();
            equipments.push(equipmentitem_extrainfo301);
        });
        packet_zc_equipment_itemlist3.set_packet_length((PacketZcEquipmentItemlist3::base_len(server_ref.packetver()) + equipments.len() * EquipmentitemExtrainfo301::base_len(server_ref.packetver())) as i16);
        packet_zc_equipment_itemlist3.set_item_info(equipments);
        packet_zc_equipment_itemlist3.fill_raw();
        let mut packet_zc_normal_itemlist3 = PacketZcNormalItemlist3::new();
        let mut normal_items = vec![];
        character.inventory_normal().iter().for_each(|(index, item)| {
            let mut extrainfo3 = NormalitemExtrainfo3::new();
            extrainfo3.set_itid(item.item_id as u16);
            extrainfo3.set_atype(item.item_type.to_client_type() as u8);
            extrainfo3.set_index(*index as i16);
            extrainfo3.set_count(item.amount);
            extrainfo3.set_is_identified(item.is_identified);
            extrainfo3.set_wear_state(item.equip as u16);
            let mut equipslotinfo = EQUIPSLOTINFO::new();
            equipslotinfo.set_card1(item.card0 as u16);
            equipslotinfo.set_card2(item.card1 as u16);
            equipslotinfo.set_card3(item.card2 as u16);
            equipslotinfo.set_card4(item.card3 as u16);
            extrainfo3.set_slot(equipslotinfo);
            extrainfo3.fill_raw();
            normal_items.push(extrainfo3);
        });
        packet_zc_normal_itemlist3.set_packet_length((PacketZcNormalItemlist3::base_len(server_ref.packetver()) + normal_items.len() * NormalitemExtrainfo3::base_len(server_ref.packetver())) as i16);
        packet_zc_normal_itemlist3.set_item_info(normal_items);
        packet_zc_normal_itemlist3.fill_raw();
        server_ref.add_to_next_tick(CharacterUpdateWeight(character.char_id));
        server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id,
                                                                                            chain_packets(vec![&packet_zc_equipment_itemlist3, &packet_zc_normal_itemlist3]))))
            .expect("Fail to send client notification");
    }

    pub fn equip_item(&self, server_ref: &Server, character: &mut Character, character_equip_item: CharacterEquipItem) {
        let mut packet_zc_req_wear_equip_ack = PacketZcReqWearEquipAck2::new();
        packet_zc_req_wear_equip_ack.set_index(character_equip_item.index as u16);
        packet_zc_req_wear_equip_ack.set_result(1);
        packet_zc_req_wear_equip_ack.set_view_id(0);

        let mut packets_raws_by_value = vec![];
        if let Some(equipped_take_off_items) = character.equip_item(character_equip_item.index) {
            packet_zc_req_wear_equip_ack.set_result(0);
            packet_zc_req_wear_equip_ack.set_wear_location(equipped_take_off_items[0].1 as u16);
            packet_zc_req_wear_equip_ack.fill_raw();
            let mut take_off_items_packets = vec![];
            if equipped_take_off_items.len() > 0 {
                for i in 1..equipped_take_off_items.len() {
                    let mut packet_zc_req_takeoff_equip_ack2 = PacketZcReqTakeoffEquipAck2::new();
                    packet_zc_req_takeoff_equip_ack2.set_index(equipped_take_off_items[i].0 as u16);
                    packet_zc_req_takeoff_equip_ack2.set_wear_location(equipped_take_off_items[i].1 as u16);
                    packet_zc_req_takeoff_equip_ack2.set_result(0);
                    packet_zc_req_takeoff_equip_ack2.fill_raw();
                    take_off_items_packets.push(packet_zc_req_takeoff_equip_ack2);
                }
            }
            packets_raws_by_value = chain_packets_raws_by_value(take_off_items_packets.iter().map(|packet| packet.raw.clone()).collect());
        }
        packets_raws_by_value.extend(packet_zc_req_wear_equip_ack.raw);
        server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets_raws_by_value)))
            .expect("Fail to send client notification");
        // check if item is equipable
        // check class requirement
        // check level requirement
        // persist in db
    }

    pub fn takeoff_equip_item(&self, server_ref: &Server, character: &mut Character, index: usize) {
        let mut packet_zc_req_takeoff_equip_ack2 = PacketZcReqTakeoffEquipAck2::new();
        packet_zc_req_takeoff_equip_ack2.set_index(index as u16);
        if let Some(location) = character.takeoff_equip_item(index) {
            packet_zc_req_takeoff_equip_ack2.set_wear_location(location as u16);
            packet_zc_req_takeoff_equip_ack2.set_result(0);
        } else {
            packet_zc_req_takeoff_equip_ack2.set_result(1);
        }
        packet_zc_req_takeoff_equip_ack2.fill_raw();
        server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_req_takeoff_equip_ack2.raw)))
            .expect("Fail to send client notification");
        // persist in db
    }
}
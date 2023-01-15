use std::sync::{Arc, Once};
use std::sync::mpsc::SyncSender;
use rand::RngCore;
use tokio::runtime::Runtime;
use enums::EnumWithMaskValue;
use enums::item::{EquipmentLocation};
use enums::look::LookType;
use crate::enums::EnumWithNumberValue;
use packets::packets::{EquipmentitemExtrainfo301, EQUIPSLOTINFO, NormalitemExtrainfo3, Packet, PacketZcEquipmentItemlist3, PacketZcItemPickupAck3, PacketZcNormalItemlist3, PacketZcPcPurchaseResult, PacketZcReqTakeoffEquipAck2, PacketZcReqWearEquipAck2, PacketZcSpriteChange2};
use crate::repository::model::item_model::{EquippedItem, InventoryItemModel, ItemModel};
use crate::repository::Repository;
use crate::server::core::configuration::Config;
use crate::server::core::tasks_queue::TasksQueue;
use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::events::game_event::{CharacterAddItems, CharacterEquipItem, CharacterZeny, GameEvent};
use crate::server::events::game_event::GameEvent::{CharacterUpdateWeight, CharacterUpdateZeny};
use crate::server::events::persistence_event::{InventoryItemUpdate, PersistenceEvent};
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::global_config_service::GlobalConfigService;


use crate::server::state::character::Character;
use crate::util::packet::{chain_packets, chain_packets_raws_by_value};

static mut SERVICE_INSTANCE: Option<InventoryService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct InventoryService{
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    repository: Arc<Repository>,
    configuration_service: &'static GlobalConfigService,
    server_task_queue: Arc<TasksQueue<GameEvent>>
}

impl InventoryService{
    pub fn instance() -> &'static InventoryService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<Repository>, configuration_service: &'static GlobalConfigService, task_queue: Arc<TasksQueue<GameEvent>>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(InventoryService{ client_notification_sender, persistence_event_sender, repository, configuration_service, server_task_queue: task_queue });
        });
    }

    pub fn add_items_in_inventory(&self, runtime: &Runtime, add_items: CharacterAddItems, character: &mut Character) {
        let mut rng = rand::thread_rng();
        let inventory_item_updates: Vec<InventoryItemUpdate> = add_items.items.iter().map(|item| {
            if item.item_type.is_stackable() {
                InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id as i32, amount: item.amount as i16, stackable: true, identified: item.is_identified, unique_id: 0 }
            } else {
                InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id as i32, amount: item.amount as i16, stackable: false, identified: item.is_identified, unique_id: rng.next_u32() as i64 }
            }
        }).collect();

        let result = runtime.block_on(async {
            self.repository.character_inventory_update(&inventory_item_updates, add_items.buy).await
        });
        if result.is_ok() {
            let mut packets = vec![];
            character.add_items(add_items.items).iter().for_each(|(index, item)| {
                let item_info = self.configuration_service.get_item(item.item_id);
                let mut packet_zc_item_pickup_ack3 = PacketZcItemPickupAck3::new();
                packet_zc_item_pickup_ack3.set_itid(item.item_id as u16);
                packet_zc_item_pickup_ack3.set_count(item.amount as u16);
                packet_zc_item_pickup_ack3.set_index(*index as u16);
                packet_zc_item_pickup_ack3.set_is_identified(item.is_identified);
                packet_zc_item_pickup_ack3.set_atype(item.item_type.to_client_type() as u8);
                packet_zc_item_pickup_ack3.set_location(item_info.location as u16);
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
                // Zeny amount have been updated in database in a single transaction with inventory update, so we need to fetch db value, then update in memory and send notification to client
                self.server_task_queue.add_to_first_index(CharacterUpdateZeny(CharacterZeny { char_id: character.char_id, zeny: None }));
            }
            self.server_task_queue.add_to_first_index(CharacterUpdateWeight(character.char_id));
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets_raws_by_value))).expect("Fail to send client notification");
        } else {
            if add_items.buy {
                let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new();
                packet_zc_pc_purchase_result.set_result(1);
                packet_zc_pc_purchase_result.fill_raw();
                self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_pc_purchase_result.raw))).expect("Fail to send client notification");
            }
            error!("{:?}", result.err());
        }
    }

    pub fn reload_inventory(&self, runtime: &Runtime, char_id: u32, character: &mut Character) {
        character.inventory = vec![];
        let _items = runtime.block_on(async {
            let items = self.repository.character_inventory_fetch(char_id as i32).await.unwrap();
            character.add_items(items)
        });
        //PacketZcNormalItemlist3
        let mut packet_zc_equipment_itemlist3 = PacketZcEquipmentItemlist3::new();
        let mut equipments = vec![];
        character.inventory_equip().iter().for_each(|(index, item)| {
            let item_info = self.configuration_service.get_item(item.item_id);
            let mut equipmentitem_extrainfo301 = EquipmentitemExtrainfo301::new();
            equipmentitem_extrainfo301.set_itid(item.item_id as u16);
            equipmentitem_extrainfo301.set_atype(item.item_type.value() as u8);
            equipmentitem_extrainfo301.set_index(*index as i16);
            equipmentitem_extrainfo301.set_is_damaged(item.is_damaged);
            equipmentitem_extrainfo301.set_is_identified(item.is_identified);
            equipmentitem_extrainfo301.set_location(item_info.location as u16);
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
        packet_zc_equipment_itemlist3.set_packet_length((PacketZcEquipmentItemlist3::base_len(self.configuration_service.packetver()) + equipments.len() * EquipmentitemExtrainfo301::base_len(self.configuration_service.packetver())) as i16);
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
        packet_zc_normal_itemlist3.set_packet_length((PacketZcNormalItemlist3::base_len(self.configuration_service.packetver()) + normal_items.len() * NormalitemExtrainfo3::base_len(self.configuration_service.packetver())) as i16);
        packet_zc_normal_itemlist3.set_item_info(normal_items);
        packet_zc_normal_itemlist3.fill_raw();
        self.server_task_queue.add_to_first_index(CharacterUpdateWeight(character.char_id));
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id,
                                                                                            chain_packets(vec![&packet_zc_equipment_itemlist3, &packet_zc_normal_itemlist3]))))
            .expect("Fail to send client notification");
    }

    pub fn reload_equipped_item_sprites(&self, character: &Character) {
        let mut packets: Vec<u8> = vec![];
        character.inventory_equipped().for_each(|(_, item)| {
            if let Some(packet) = self.sprite_change_packet_for_item(character, item) {
                packets.extend(packet);
            }
        });
        CharacterService::instance().send_area_notification_around_characters(character, packets);
    }

    pub fn sprite_change_packet_for_item(&self, character: &Character, item: &InventoryItemModel) -> Option<Vec<u8>> {
        let mut packet_zc_sprite_change = PacketZcSpriteChange2::new();
        packet_zc_sprite_change.set_gid(character.char_id);
        let item_info = self.configuration_service.get_item(item.item_id);
        if item.equip & EquipmentLocation::HandRight.as_flag() as i32 != 0 {
            packet_zc_sprite_change.set_atype(LookType::Weapon.value() as u8);
            packet_zc_sprite_change.set_value(item_info.view.unwrap_or(item.item_id) as u16);
        }
        if item.equip & EquipmentLocation::HandLeft.as_flag() as i32 != 0 {
            packet_zc_sprite_change.set_atype(LookType::Shield.value() as u8);
            packet_zc_sprite_change.set_value2(item_info.view.unwrap_or(item.item_id) as u16);
        }
        if item.equip & EquipmentLocation::HeadTop.as_flag() as i32 != 0 {
            packet_zc_sprite_change.set_atype(LookType::HeadTop.value() as u8);
            packet_zc_sprite_change.set_value(item_info.view.unwrap_or(item.item_id) as u16);
        }
        if item.equip & EquipmentLocation::HeadMid.as_flag() as i32 != 0 {
            packet_zc_sprite_change.set_atype(LookType::HeadMid.value() as u8);
            packet_zc_sprite_change.set_value(item_info.view.unwrap_or(item.item_id) as u16);
        }
        if item.equip & EquipmentLocation::HeadLow.as_flag() as i32 != 0 {
            packet_zc_sprite_change.set_atype(LookType::HeadBottom.value() as u8);
            packet_zc_sprite_change.set_value(item_info.view.unwrap_or(item.item_id) as u16);
        }
        if packet_zc_sprite_change.atype != 0 {
            packet_zc_sprite_change.fill_raw();
            return Some(packet_zc_sprite_change.raw);
        }
        None
    }

    pub fn equip_item<'a>(&self, character: &'a mut Character, character_equip_item: CharacterEquipItem) {
        let mut packet_zc_req_wear_equip_ack = PacketZcReqWearEquipAck2::new();
        let index = character_equip_item.index;
        packet_zc_req_wear_equip_ack.set_index(index as u16);
        packet_zc_req_wear_equip_ack.set_result(1);
        packet_zc_req_wear_equip_ack.set_view_id(0);
        packet_zc_req_wear_equip_ack.set_wear_location(0);

        let mut packets_raws_by_value = vec![];

        if let Some(inventory_item) = character.get_item_from_inventory(index) {
            let equip_item = self.configuration_service.get_item(inventory_item.item_id);
            let location = equip_item.location as i32; // it won't work for shadow gear
            let item_id = equip_item.id;
            let mut equipped_take_off_items: Vec<EquippedItem> = vec![];
            if !equip_item.item_type.is_equipment() {
                return;
            }
            if self.check_weapon_requirements(character, equip_item) {
                if location & EquipmentLocation::AccessoryLeft.as_flag() as i32 != 0 || location & EquipmentLocation::AccessoryRight.as_flag() as i32 != 0 {
                    // Remove equipped accessory if both(right and left) slots are occupied, otherwise just equip the item in the free slot (right or left)
                    let accessories: Vec<(usize, &InventoryItemModel)> = character.inventory.iter().enumerate()
                        .filter(|(_, i)| if let Some(j) = i { j.item_type.is_equipment() && (j.equip & location != 0) } else { false })
                        .map(|(index, item)| (index, item.as_ref().unwrap()))
                        .collect();
                    if accessories.len() == 2 {
                        equipped_take_off_items.push(EquippedItem { item_id, removed_equip_location: EquipmentLocation::AccessoryLeft.as_flag() as i32, index });
                        // When the 2 accessories slot are occupied, remove left accessory and equip new one in the left slot
                        let (item_to_remove_index, _) = accessories.iter().find(|(_index, item)| item.equip & EquipmentLocation::AccessoryLeft.as_flag() as i32 != 0).unwrap();
                        let item_to_remove_index = *item_to_remove_index;
                        drop(accessories);
                        let mut item = character.get_item_from_inventory_mut(item_to_remove_index).unwrap();
                        equipped_take_off_items.push(EquippedItem { item_id: item.item_id, removed_equip_location: item.equip, index: item_to_remove_index });
                        item.equip = 0;
                        character.get_item_from_inventory_mut(index).unwrap().equip = EquipmentLocation::AccessoryLeft.as_flag() as i32;
                    } else if accessories.len() == 1 {
                        // When only 1 accessory slot is occupied, equip the new item in the free slot
                        vec![EquipmentLocation::AccessoryRight.as_flag() as i32, EquipmentLocation::AccessoryLeft.as_flag() as i32].iter()
                            .find(|item_mask| accessories[0].1.equip & **item_mask == 0)
                            .map(|item_mask| {
                                equipped_take_off_items.push(EquippedItem { item_id, removed_equip_location: *item_mask as i32, index });
                                character.get_item_from_inventory_mut(index).unwrap().equip = *item_mask as i32;
                            });
                    } else {
                        equipped_take_off_items.push(EquippedItem { item_id, removed_equip_location: EquipmentLocation::AccessoryLeft.as_flag() as i32, index });
                        character.get_item_from_inventory_mut(index).unwrap().equip = EquipmentLocation::AccessoryLeft.as_flag() as i32;
                    }
                } else {
                    equipped_take_off_items.push(EquippedItem { item_id, removed_equip_location: location, index });
                    // Remove equipped items in same location. E.g: when goggle item is equipped it remove top and mid head items, when a 2h weapon is equipped it remove shield and weapon items...
                    character.inventory.iter_mut().enumerate()
                        .filter(|(_, i)| if let Some(j) = i { j.item_type.is_equipment() && (j.equip & location != 0) } else { false })
                        .for_each(|(item_index, inventory_item)| {
                            let inventory_item = inventory_item.as_mut().unwrap();
                            equipped_take_off_items.push(EquippedItem { item_id: inventory_item.id, removed_equip_location: inventory_item.equip, index: item_index });
                            inventory_item.equip = 0;
                        });
                    character.get_item_from_inventory_mut(index).unwrap().equip = location;
                }
                let item_view = equip_item.view.unwrap_or(0);
                packet_zc_req_wear_equip_ack.set_view_id(item_view as u16);
                packet_zc_req_wear_equip_ack.set_result(0);
                packet_zc_req_wear_equip_ack.set_wear_location(equipped_take_off_items[0].removed_equip_location as u16);
                let mut take_off_items_packets = vec![];
                if !equipped_take_off_items.is_empty() {
                    for i in 1..equipped_take_off_items.len() {
                        let mut packet_zc_req_takeoff_equip_ack2 = PacketZcReqTakeoffEquipAck2::new();
                        packet_zc_req_takeoff_equip_ack2.set_index(equipped_take_off_items[i].index as u16);
                        packet_zc_req_takeoff_equip_ack2.set_wear_location(equipped_take_off_items[i].removed_equip_location as u16);
                        packet_zc_req_takeoff_equip_ack2.set_result(0);
                        packet_zc_req_takeoff_equip_ack2.fill_raw();
                        take_off_items_packets.push(packet_zc_req_takeoff_equip_ack2);
                    }
                }
                packets_raws_by_value = chain_packets_raws_by_value(take_off_items_packets.iter().map(|packet| packet.raw.clone()).collect());
            }
        }
        packet_zc_req_wear_equip_ack.fill_raw();
        packets_raws_by_value.extend(packet_zc_req_wear_equip_ack.raw);
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets_raws_by_value)))
            .expect("Fail to send client notification");
        self.persistence_event_sender.send(PersistenceEvent::UpdateEquippedItems(character.inventory_wearable().iter().cloned().map(|(_m, item)| item.clone()).collect::<Vec<InventoryItemModel>>()))
            .expect("Fail to send persistence event");

        // check if item is equipable
        // check class requirement
        // check level requirement
    }

    pub fn check_weapon_requirements(&self, character: &mut Character, equip_item: &ItemModel) -> bool {
        character.status.base_level >= (equip_item.equip_level_min.unwrap_or(0) as u32)
    }

    pub fn takeoff_equip_item(&self, character: &mut Character, index: usize) {
        let mut packet_zc_req_takeoff_equip_ack2 = PacketZcReqTakeoffEquipAck2::new();
        packet_zc_req_takeoff_equip_ack2.set_index(index as u16);
        if let Some(location) = character.takeoff_equip_item(index) {
            packet_zc_req_takeoff_equip_ack2.set_wear_location(location as u16);
            packet_zc_req_takeoff_equip_ack2.set_result(0);
        } else {
            packet_zc_req_takeoff_equip_ack2.set_result(1);
        }
        packet_zc_req_takeoff_equip_ack2.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_req_takeoff_equip_ack2.raw)))
            .expect("Fail to send client notification");
        self.persistence_event_sender.send(PersistenceEvent::UpdateEquippedItems(character.inventory_wearable().iter().cloned().map(|(_m, item)| item.clone()).collect::<Vec<InventoryItemModel>>()))
            .expect("Fail to send persistence event");
    }
}
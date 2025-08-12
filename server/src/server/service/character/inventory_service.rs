use crate::repository::model::item_model::{InventoryItemModel, ItemModel};
use crate::repository::InventoryRepository;
use models::enums::class::{EquipClassFlag, JobName};
use models::enums::item::{EquipmentLocation, ItemType};
use models::enums::look::LookType;
use models::enums::EnumWithMaskValueU64;
use models::enums::EnumWithNumberValue;
use models::item::Wearable;
use packets::packets::{EquipmentitemExtrainfo301, NormalitemExtrainfo3, Packet, PacketZcAttackRange, PacketZcEquipArrow, PacketZcEquipmentItemlist3, PacketZcItemFallEntry, PacketZcItemPickupAck3, PacketZcItemThrowAck, PacketZcItemcompositionList, PacketZcNormalItemlist3, PacketZcPcPurchaseResult, PacketZcReqTakeoffEquipAck2, PacketZcReqWearEquipAck2, PacketZcSkillinfoList, PacketZcSpriteChange2, EQUIPSLOTINFO, SKILLINFO};
use rand::RngCore;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::game_event::GameEvent::{CharacterUpdateWeight, CharacterUpdateZeny};
use crate::server::model::events::game_event::{CharacterAddItems, CharacterEquipItem, CharacterRemoveItem, CharacterRemoveItems, CharacterRequestCardCompositionList, CharacterZeny, GameEvent};
use crate::server::model::events::map_event::{CharacterDropItems, MapEvent};
use crate::server::model::events::persistence_event::{InventoryItemUpdate, PersistenceEvent};
use crate::server::model::map_instance::MapInstance;
use crate::server::model::tasks_queue::TasksQueue;
use models::item::EquippedItem;

use crate::server::service::global_config_service::GlobalConfigService;


use crate::server::state::character::Character;
use crate::util::packet::{chain_packets, chain_packets_raws_by_value};

pub struct InventoryService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    repository: Arc<dyn InventoryRepository + Sync>,
    configuration_service: &'static GlobalConfigService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

impl InventoryService {

    pub fn new(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<dyn InventoryRepository + Sync>, configuration_service: &'static GlobalConfigService, task_queue: Arc<TasksQueue<GameEvent>>) -> Self {
        Self { client_notification_sender, persistence_event_sender, repository, configuration_service, server_task_queue: task_queue }
    }

    pub fn add_items_in_inventory(&self, runtime: &Runtime, add_items: CharacterAddItems, character: &mut Character) {
        let mut rng = rand::thread_rng();
        let inventory_item_updates: Vec<InventoryItemUpdate> = add_items.items.iter().map(|item| {
            if item.item_type().is_stackable() {
                InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id, amount: item.amount, stackable: true, identified: item.is_identified, unique_id: 0 }
            } else {
                InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id, amount: item.amount, stackable: false, identified: item.is_identified, unique_id: rng.next_u32() as i64 }
            }
        }).collect();

        let result = runtime.block_on(async {
            self.repository.character_inventory_update_add(&inventory_item_updates, add_items.buy).await
        });
        if result.is_ok() {
            let mut packets = vec![];
            character.add_items(add_items.items).iter().for_each(|(index, item)| {
                let item_info = self.configuration_service.get_item(item.item_id);
                let mut packet_zc_item_pickup_ack3 = PacketZcItemPickupAck3::new(self.configuration_service.packetver());
                packet_zc_item_pickup_ack3.set_itid(item.item_id as u16);
                packet_zc_item_pickup_ack3.set_count(item.amount as u16);
                packet_zc_item_pickup_ack3.set_index(*index as u16);
                packet_zc_item_pickup_ack3.set_is_identified(item.is_identified);
                packet_zc_item_pickup_ack3.set_atype(item.item_type().to_client_type() as u8);
                packet_zc_item_pickup_ack3.set_location(item_info.location as u16);
                packet_zc_item_pickup_ack3.fill_raw();
                packets.push(packet_zc_item_pickup_ack3)
            });
            let mut packets_raws_by_value = chain_packets_raws_by_value(packets.iter().map(|packet| packet.raw.clone()).collect());
            if add_items.buy {
                let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new(self.configuration_service.packetver());
                packet_zc_pc_purchase_result.set_result(0);
                packet_zc_pc_purchase_result.fill_raw();
                packets_raws_by_value.extend(packet_zc_pc_purchase_result.raw);
                // Zeny amount have been updated in database in a single transaction with inventory update, so we need to fetch db value, then update in memory and send notification to client
                self.server_task_queue.add_to_first_index(CharacterUpdateZeny(CharacterZeny { char_id: character.char_id, zeny: None }));
            }
            self.server_task_queue.add_to_first_index(CharacterUpdateWeight(character.char_id));
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets_raws_by_value)))
                .unwrap_or_else(|_| error!("Failed to send notification packet_zc_pc_purchase_result to client"));
        } else {
            if add_items.buy {
                let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new(self.configuration_service.packetver());
                packet_zc_pc_purchase_result.set_result(1);
                packet_zc_pc_purchase_result.fill_raw();
                self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_pc_purchase_result.raw)))
                    .unwrap_or_else(|_| error!("Failed to send notification packet_zc_pc_purchase_result to client"));
            }
            error!("{:?}", result.err());
        }
    }

    pub fn character_sell_items(&self,  runtime: &Runtime, character: &mut Character, remove_items: CharacterRemoveItems) {
        self.remove_item_from_inventory(runtime, remove_items, character).unwrap();
    }

    pub fn character_drop_items(&self,  runtime: &Runtime, character: &mut Character, remove_items: CharacterRemoveItems, map_instance: &MapInstance) {
        if let Ok(inventory_items) = self.remove_item_from_inventory(runtime, remove_items, character) {
            map_instance.add_to_next_tick(MapEvent::CharDropItems(CharacterDropItems{ owner_id: character.char_id, char_x: character.x, char_y: character.y, item_removal_info: inventory_items }));
        }
    }

    pub fn remove_item_from_inventory(&self, runtime: &Runtime, remove_items: CharacterRemoveItems, character: &mut Character) -> Result<Vec<(InventoryItemModel, CharacterRemoveItem)>, String> {
        let mut items = Vec::with_capacity(remove_items.items.len());
        for remove_item in remove_items.items.iter() {
            if let Some(item) = character.get_item_from_inventory(remove_item.index) {
                let mut item = item.clone();
                item.amount -= remove_item.amount;
                items.push((item, *remove_item));
            }
        }
        let result = runtime.block_on(async {
            self.repository.character_inventory_update_remove(&items, remove_items.sell).await
        });

        if result.is_ok() {
            let mut packets = vec![];
            for remove_item in remove_items.items.iter() {
                character.del_item_from_inventory(remove_item.index, remove_item.amount);
                let mut packet_zc_item_throw_ack = PacketZcItemThrowAck::new(self.configuration_service.packetver());
                packet_zc_item_throw_ack.set_index(remove_item.index as u16);
                packet_zc_item_throw_ack.set_count(remove_item.amount);
                packet_zc_item_throw_ack.fill_raw();
                packets.push(packet_zc_item_throw_ack.raw);
            }
            for (item, remove_item) in items.iter() {
                let mut packet_zc_item_fall_entry = PacketZcItemFallEntry::new(self.configuration_service.packetver());
                packet_zc_item_fall_entry.set_itid(item.item_id as u16);
                packet_zc_item_fall_entry.set_itaid(item.id as u32);
                packet_zc_item_fall_entry.set_count(remove_item.amount);

            }
            self.server_task_queue.add_to_first_index(CharacterUpdateWeight(character.char_id));
            if remove_items.sell {
                let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new(self.configuration_service.packetver());
                packet_zc_pc_purchase_result.set_result(0);
                packet_zc_pc_purchase_result.fill_raw();
                packets.push(packet_zc_pc_purchase_result.raw);
                self.server_task_queue.add_to_first_index(CharacterUpdateZeny(CharacterZeny { char_id: character.char_id, zeny: None }));
            }
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, chain_packets_raws_by_value(packets))))
                .unwrap_or_else(|_| error!("Failed to send notification packet_zc_pc_purchase_result to client"));
            Ok(items)
        } else if remove_items.sell {
            let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new(self.configuration_service.packetver());
            packet_zc_pc_purchase_result.set_result(1);
            packet_zc_pc_purchase_result.fill_raw();
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_pc_purchase_result.raw)))
                .unwrap_or_else(|_| error!("Failed to send notification packet_zc_pc_purchase_result to client"));
            Err("Cannot sell items".to_string())
        } else {
            Err("Cannot drop item".to_string())
        }
    }

    pub fn reload_inventory(&self, runtime: &Runtime, char_id: u32, character: &mut Character) {
        character.inventory = vec![];
        runtime.block_on(async {
            let items = self.repository.character_inventory_fetch(char_id as i32).await.unwrap();
            character.status.takeoff_all_equipment();
            character.add_items(items);
        });
        //PacketZcNormalItemlist3
        let mut packet_zc_equipment_itemlist3 = PacketZcEquipmentItemlist3::new(self.configuration_service.packetver());
        let mut equipments = vec![];
        for (index, item) in character.inventory_equip().iter() {
            let item_info = self.configuration_service.get_item(item.item_id);
            let mut equipmentitem_extrainfo301 = EquipmentitemExtrainfo301::new(self.configuration_service.packetver());
            equipmentitem_extrainfo301.set_itid(item.item_id as u16);
            equipmentitem_extrainfo301.set_atype(item.item_type().value() as u8);
            equipmentitem_extrainfo301.set_index(*index as i16);
            equipmentitem_extrainfo301.set_is_damaged(item.is_damaged);
            equipmentitem_extrainfo301.set_is_identified(item.is_identified);
            equipmentitem_extrainfo301.set_location(item_info.location as u16);
            equipmentitem_extrainfo301.set_wear_state(item.equip as u16);
            equipmentitem_extrainfo301.set_refining_level(item.refine as u8);
            let mut equipslotinfo = EQUIPSLOTINFO::new(self.configuration_service.packetver());
            equipslotinfo.set_card1(item.card0 as u16);
            equipslotinfo.set_card2(item.card1 as u16);
            equipslotinfo.set_card3(item.card2 as u16);
            equipslotinfo.set_card4(item.card3 as u16);
            equipmentitem_extrainfo301.set_slot(equipslotinfo);
            equipmentitem_extrainfo301.fill_raw();
            equipments.push(equipmentitem_extrainfo301);
        }
        let mut ammo: Option<(usize, &InventoryItemModel)> = None;
        for (index, item) in character.inventory_wearable().iter() {
            if matches!(item.item_type(), ItemType::Ammo) {
                ammo = Some((*index, item));
                break;
            }
        }
        let mut maybe_packet_zc_equip_arrow = None;
        if let Some((index, ammo)) = ammo {
            let mut packet_zc_equip_arrow = PacketZcEquipArrow::new(self.configuration_service.packetver());
            let item_info = self.configuration_service.get_item(ammo.item_id);
            character.wear_equip_item(index, ammo.equip as u64, item_info);
            packet_zc_equip_arrow.set_index(index as i16);
            packet_zc_equip_arrow.fill_raw();
            maybe_packet_zc_equip_arrow = Some(packet_zc_equip_arrow.raw);
        }
        for item in equipments.iter() {
            let item_info = self.configuration_service.get_item(item.itid as i32);
            character.wear_equip_item(item.index as usize, item.wear_state as u64, item_info);
        }
        packet_zc_equipment_itemlist3.set_packet_length((PacketZcEquipmentItemlist3::base_len(self.configuration_service.packetver()) + equipments.len() * EquipmentitemExtrainfo301::base_len(self.configuration_service.packetver())) as i16);
        packet_zc_equipment_itemlist3.set_item_info(equipments);
        packet_zc_equipment_itemlist3.fill_raw();
        let mut packet_zc_normal_itemlist3 = PacketZcNormalItemlist3::new(self.configuration_service.packetver());
        let mut normal_items = vec![];
        character.inventory_normal().iter().for_each(|(index, item)| {
            let mut extrainfo3 = NormalitemExtrainfo3::new(self.configuration_service.packetver());
            extrainfo3.set_itid(item.item_id as u16);
            extrainfo3.set_atype(item.item_type().to_client_type() as u8);
            extrainfo3.set_index(*index as i16);
            extrainfo3.set_count(item.amount);
            extrainfo3.set_is_identified(item.is_identified);
            extrainfo3.set_wear_state(item.equip as u16);
            let mut equipslotinfo = EQUIPSLOTINFO::new(self.configuration_service.packetver());
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
        let packet_zc_attack_range = self.packet_attack_range(character);
        self.server_task_queue.add_to_first_index(CharacterUpdateWeight(character.char_id));
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id,
                                                                                      chain_packets(vec![&packet_zc_equipment_itemlist3, &packet_zc_normal_itemlist3, &packet_zc_attack_range]))))
            .unwrap_or_else(|_| error!("Failed to send notification packet_zc_normal_itemlist3 to client"));
        if let Some(packet) = maybe_packet_zc_equip_arrow {
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet)))
                .unwrap_or_else(|_| error!("Failed to send notification equip arrow to client"));
        }
    }

    fn packet_attack_range(&self, character: &mut Character) -> PacketZcAttackRange {
        let mut packet_zc_attack_range = PacketZcAttackRange::new(self.configuration_service.packetver());
        packet_zc_attack_range.set_current_att_range(character.status.attack_range() as i16);
        packet_zc_attack_range.fill_raw();
        packet_zc_attack_range
    }

    pub fn reload_equipped_item_sprites(&self, character: &Character) {
        let mut packets: Vec<u8> = vec![];
        character.status.equipped_gears().iter().for_each(|item| {
            if let Some(packet) = self.sprite_change_packet_for_item(character, item, false) {
                packets.extend(packet);
            }
        });
        character.status.equipped_weapons().iter().for_each(|item| {
            if let Some(packet) = self.sprite_change_packet_for_item(character, item, false) {
                packets.extend(packet);
            }
        });
        self.client_notification_sender.send(Notification::Area(AreaNotification {
            map_name: character.current_map_name().clone(),
            map_instance_id: character.current_map_instance(),
            range_type: AreaNotificationRangeType::Fov { x: character.x(), y: character.y(), exclude_id: None },
            packet: packets,
        })).unwrap_or_else(|_| error!("Failed to send notification reload equipment sprite to client"));
    }

    pub fn sprite_change_packet_for_item(&self, character: &Character, item: &dyn Wearable, is_takeoff: bool) -> Option<Vec<u8>> {
        let mut packet_zc_sprite_change = PacketZcSpriteChange2::new(self.configuration_service.packetver());
        packet_zc_sprite_change.set_gid(character.char_id);
        let item_info = self.configuration_service.get_item(item.item_id());
        if item.location() & EquipmentLocation::HandRight.as_flag() > 0 {
            packet_zc_sprite_change.set_atype(LookType::Weapon.value() as u8);
            packet_zc_sprite_change.set_value(item_info.view.unwrap_or(item.item_id()) as u16);
        }
        else if item.location() & EquipmentLocation::HandLeft.as_flag() > 0 {
            packet_zc_sprite_change.set_atype(LookType::Shield.value() as u8);
            packet_zc_sprite_change.set_value2(item_info.view.unwrap_or(item.item_id()) as u16);
        }
        else if item.location() & EquipmentLocation::HeadTop.as_flag() > 0 {
            packet_zc_sprite_change.set_atype(LookType::HeadTop.value() as u8);
            packet_zc_sprite_change.set_value(item_info.view.unwrap_or(item.item_id()) as u16);
        }
        else if item.location() & EquipmentLocation::HeadMid.as_flag() > 0 {
            packet_zc_sprite_change.set_atype(LookType::HeadMid.value() as u8);
            packet_zc_sprite_change.set_value(item_info.view.unwrap_or(item.item_id()) as u16);
        }
        else if item.location() & EquipmentLocation::HeadLow.as_flag() > 0 {
            packet_zc_sprite_change.set_atype(LookType::HeadBottom.value() as u8);
            packet_zc_sprite_change.set_value(item_info.view.unwrap_or(item.item_id()) as u16);
        }
        if is_takeoff {
            packet_zc_sprite_change.set_value(0_u16);
        }
         if packet_zc_sprite_change.atype != 0 {
            packet_zc_sprite_change.fill_raw();
            return Some(packet_zc_sprite_change.raw);
        }
        None
    }

    pub fn equip_item(&self, character: &mut Character, character_equip_item: CharacterEquipItem) -> Option<EquippedItem> {
        let mut packet_zc_req_wear_equip_ack = PacketZcReqWearEquipAck2::new(self.configuration_service.packetver());
        let mut packet_zc_equip_arrow = PacketZcEquipArrow::new(self.configuration_service.packetver());
        let index = character_equip_item.index;
        packet_zc_req_wear_equip_ack.set_index(index as u16);
        packet_zc_req_wear_equip_ack.set_result(1);
        packet_zc_req_wear_equip_ack.set_view_id(0);
        packet_zc_req_wear_equip_ack.set_wear_location(0);
        packet_zc_equip_arrow.set_index(index as i16);

        let mut packets_raws_by_value = vec![];

        let mut equipped_item = None;
        if let Some(inventory_item) = character.get_item_from_inventory(index) {
            let item_to_equip_model = self.configuration_service.get_item(inventory_item.item_id);
            let location = item_to_equip_model.location as i32; // it won't work for shadow gear
            let item_id = item_to_equip_model.id;
            let mut equipped_take_off_items: Vec<EquippedItem> = vec![];
            if !item_to_equip_model.item_type.is_wearable() {
                return None;
            }
            // TODO check if can carry (< 90% weight)
            if self.check_base_level_requirement(character, item_to_equip_model) && self.check_job_requirement(character, item_to_equip_model) {
                if location & EquipmentLocation::AccessoryLeft.as_flag() as i32 != 0 || location & EquipmentLocation::AccessoryRight.as_flag() as i32 != 0 {
                    // Remove equipped accessory if both(right and left) slots are occupied, otherwise just equip the item in the free slot (right or left)
                    let accessories: Vec<(usize, &InventoryItemModel)> = character.inventory.iter().enumerate()
                        .filter(|(_, i)| if let Some(j) = i { j.item_type().is_equipment() && (j.equip & location != 0) } else { false })
                        .map(|(index, item)| (index, item.as_ref().unwrap()))
                        .collect();
                    if accessories.len() == 2 {
                        equipped_take_off_items.push(EquippedItem { item_id, location: EquipmentLocation::AccessoryLeft.as_flag(), index });
                        // When the 2 accessories slot are occupied, remove left accessory and equip new one in the left slot
                        let (item_to_remove_index, _) = accessories.iter().find(|(_index, item)| item.equip & EquipmentLocation::AccessoryLeft.as_flag() as i32 != 0).unwrap();
                        let item_to_remove_index = *item_to_remove_index;
                        drop(accessories);
                        let item = character.get_item_from_inventory(item_to_remove_index).unwrap();
                        equipped_take_off_items.push(EquippedItem { item_id: item.item_id, location: item.equip as u64, index: item_to_remove_index });
                        character.takeoff_equip_item(item_to_remove_index);
                        equipped_item = character.wear_equip_item(index, EquipmentLocation::AccessoryLeft.as_flag(), item_to_equip_model);
                    } else if accessories.len() == 1 {
                        // When only 1 accessory slot is occupied, equip the new item in the free slot
                        [EquipmentLocation::AccessoryRight.as_flag() as i32, EquipmentLocation::AccessoryLeft.as_flag() as i32].iter()
                            .find(|item_mask| accessories[0].1.equip & **item_mask == 0)
                            .map(|item_mask| {
                                equipped_take_off_items.push(EquippedItem { item_id, location: *item_mask as u64, index });
                                equipped_item = character.wear_equip_item(index, *item_mask as u64, item_to_equip_model);
                            });
                    } else {
                        equipped_take_off_items.push(EquippedItem { item_id, location: EquipmentLocation::AccessoryLeft.as_flag(), index });
                        equipped_item = character.wear_equip_item(index, EquipmentLocation::AccessoryLeft.as_flag(), item_to_equip_model);
                    }
                } else {
                    equipped_take_off_items.push(EquippedItem { item_id, location: location as u64, index });
                    // Remove equipped items in same location. E.g: when goggle item is equipped it remove top and mid head items, when a 2h weapon is equipped it remove shield and weapon items...
                    let mut items_index_to_remove = vec![];
                    character.inventory.iter().enumerate()
                        .filter(|(_, i)| if let Some(j) = i { j.item_type().is_equipment() && (j.equip & location != 0) } else { false })
                        .for_each(|(item_index, inventory_item)| {
                            let inventory_item = inventory_item.as_ref().unwrap();
                            equipped_take_off_items.push(EquippedItem { item_id: inventory_item.id, location: inventory_item.equip as u64, index: item_index });
                            items_index_to_remove.push(item_index);
                        });
                    items_index_to_remove.iter().for_each(|index| {character.takeoff_equip_item(*index);});
                    equipped_item = character.wear_equip_item(index, location as u64, item_to_equip_model);
                }
                let item_view = item_to_equip_model.view.unwrap_or(0);
                packet_zc_req_wear_equip_ack.set_view_id(item_view as u16);
                packet_zc_req_wear_equip_ack.set_result(0);
                packet_zc_req_wear_equip_ack.set_wear_location(equipped_take_off_items[0].location as u16);
                let mut take_off_items_packets = vec![];
                if !equipped_take_off_items.is_empty() {
                    for i in 1..equipped_take_off_items.len() {
                        let mut packet_zc_req_takeoff_equip_ack2 = PacketZcReqTakeoffEquipAck2::new(self.configuration_service.packetver());
                        packet_zc_req_takeoff_equip_ack2.set_index(equipped_take_off_items[i].index as u16);
                        packet_zc_req_takeoff_equip_ack2.set_wear_location(equipped_take_off_items[i].location as u16);
                        packet_zc_req_takeoff_equip_ack2.set_result(0);
                        packet_zc_req_takeoff_equip_ack2.fill_raw();
                        take_off_items_packets.push(packet_zc_req_takeoff_equip_ack2);
                    }
                }
                packets_raws_by_value = chain_packets_raws_by_value(take_off_items_packets.iter().map(|packet| packet.raw.clone()).collect());
            }
            if matches!(item_to_equip_model.item_type, ItemType::Ammo) {
                packet_zc_equip_arrow.fill_raw();
                packets_raws_by_value.extend(packet_zc_equip_arrow.raw);
            } else {
                packet_zc_req_wear_equip_ack.fill_raw();
                packets_raws_by_value.extend(packet_zc_req_wear_equip_ack.raw);
            }
            if matches!(item_to_equip_model.item_type, ItemType::Weapon) {
                let packet_zc_attack_range = self.packet_attack_range(character);
                packets_raws_by_value.extend(packet_zc_attack_range.raw);
            }
            if let Some(equipped_item) = equipped_item.as_ref() {
                if let Some(script_change_packet) = self.sprite_change_packet_for_item(character, equipped_item, false) { packets_raws_by_value.extend(script_change_packet); }
            }
        } else {
            packet_zc_req_wear_equip_ack.fill_raw();
            packets_raws_by_value.extend(packet_zc_req_wear_equip_ack.raw);
        }
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets_raws_by_value)))
            .unwrap_or_else(|_| error!("Failed to send notification equip item to client"));
        self.persistence_event_sender.send(PersistenceEvent::UpdateEquippedItems(character.inventory_wearable().iter().cloned().map(|(_m, item)| item.clone()).collect::<Vec<InventoryItemModel>>()))
            .expect("Fail to send persistence event");
        self.server_task_queue.add_to_first_index(GameEvent::CharacterUpdateClientSideStats(character.char_id));
        equipped_item
    }

    pub fn check_base_level_requirement(&self, character: &Character, equip_item: &ItemModel) -> bool {
        character.status.base_level >= (equip_item.equip_level_min.unwrap_or(0) as u32)
    }
    pub fn check_job_requirement(&self, character: &Character, equip_item: &ItemModel) -> bool {
        let equip_class_flag = EquipClassFlag::flag_from_job_name(JobName::from_value(character.status.job as usize));
        equip_item.job_flags & equip_class_flag != 0
    }

    pub fn takeoff_equip_item(&self, character: &mut Character, index: usize) -> Option<EquippedItem> {
        let mut packet_zc_req_takeoff_equip_ack2 = PacketZcReqTakeoffEquipAck2::new(self.configuration_service.packetver());
        packet_zc_req_takeoff_equip_ack2.set_index(index as u16);
        let takeoff_equipement = character.takeoff_equip_item(index);
        let mut packets_raws_by_value = vec![];
        if let Some(takeoff_equipement) = takeoff_equipement.as_ref() {
            packet_zc_req_takeoff_equip_ack2.set_wear_location(takeoff_equipement.location() as u16);
            packet_zc_req_takeoff_equip_ack2.set_result(0);

            if takeoff_equipement.location & EquipmentLocation::HandRight.as_flag() > 0 || takeoff_equipement.location & EquipmentLocation::HandLeft.as_flag() > 0 {
                let packet_zc_attack_range = self.packet_attack_range(character);
                packets_raws_by_value.extend(packet_zc_attack_range.raw);
            }
            if let Some(script_change_packet) = self.sprite_change_packet_for_item(character, takeoff_equipement, true) { packets_raws_by_value.extend(script_change_packet); }
        } else {
            packet_zc_req_takeoff_equip_ack2.set_result(1);
        }
        packet_zc_req_takeoff_equip_ack2.fill_raw();
        packets_raws_by_value.extend(packet_zc_req_takeoff_equip_ack2.raw);
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets_raws_by_value)))
            .unwrap_or_else(|_| error!("Failed to send notification takeoff item to client"));
        self.persistence_event_sender.send(PersistenceEvent::UpdateEquippedItems(character.inventory_wearable().iter().cloned().map(|(_m, item)| item.clone()).collect::<Vec<InventoryItemModel>>()))
            .expect("Fail to send persistence event");
        self.server_task_queue.add_to_first_index(GameEvent::CharacterUpdateClientSideStats(character.char_id));
        takeoff_equipement
    }

    pub fn send_card_composition_list(&self, character: &mut Character, char_equip_item: CharacterEquipItem) {
        let mut packet_zc_item_composition_list = PacketZcItemcompositionList::new(self.configuration_service.packetver());

        let mut slotable_items: Vec<u16> = vec![];
        
        if let Some(card_item) = character.get_item_from_inventory(char_equip_item.index) {
            let card_info = self.configuration_service.get_item(card_item.item_id);

            if card_info.item_type.is_card() {
                for (index, inventory_item) in character.inventory.iter().enumerate() {
                    if let Some(item) = inventory_item {
                        let item_info = self.configuration_service.get_item(item.item_id);

                        if item_info.item_type.is_equipment() && card_info.location & item_info.location != 0 {
                            info!("Index {index}, {}", item_info.name_aegis);
                            slotable_items.push(index as u16);
                        }
                    }
                }
            }
        }

        packet_zc_item_composition_list.set_packet_length((PacketZcItemcompositionList::base_len(self.configuration_service.packetver()) + (slotable_items.len() * 2)) as i16);
        packet_zc_item_composition_list.set_itidlist(slotable_items);
        packet_zc_item_composition_list.fill_raw();
        
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_item_composition_list.raw)))
            .unwrap_or_else(|_| error!("Failed to send PacketZcItemcompositionList to client"));
    }
}
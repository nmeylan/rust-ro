use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::{Scope, sleep};
use std::time::{Duration, Instant};
use packets::packets::{Packet, PacketZcNpcackMapmove};
use crate::server::core::position::Position;
use crate::server::core::event::Event;
use crate::server::core::map::{MAP_EXT, MapItem, ToMapItem};
use crate::server::core::notification::{CharNotification, Notification};
use crate::server::enums::map_item::MapItemType;
use crate::server::server::Server;
use crate::util::string::StringUtil;

impl Server {
    pub(crate) fn game_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>) {
        loop {
            let start = Instant::now();
            let mut characters = server_ref.characters.borrow_mut();
            if let Some(tasks) = server_ref.pop_task() {
                for task in tasks {
                    match task {
                        Event::CharacterChangeMap(event) => {
                            let character = characters.get_mut(&event.char_id).unwrap();
                            if let Some(map_instance) = server_ref.get_map_instance(&event.new_map_name, event.new_instance_id) {
                                character.join_and_set_map(map_instance);
                                let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();

                                let mut new_current_map: [char; 16] = [0 as char; 16];
                                let map_name = format!("{}{}", event.new_map_name, MAP_EXT);
                                map_name.fill_char_array(new_current_map.as_mut());
                                packet_zc_npcack_mapmove.set_map_name(new_current_map);
                                packet_zc_npcack_mapmove.set_x_pos(character.x() as i16);
                                packet_zc_npcack_mapmove.set_y_pos(character.y() as i16);
                                packet_zc_npcack_mapmove.fill_raw();
                                client_notification_sender_clone.send(Notification::Char(CharNotification::new(character.account_id, std::mem::take(packet_zc_npcack_mapmove.raw_mut()))))
                                    .expect("Failed to send notification event with PacketZcNpcackMapmove");
                            }
                        }
                        Event::CharacterRemoveFromMap(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            if let Some(map_instance) = server_ref.get_map_instance(character.current_map_name(), character.current_map_instance()) {
                                map_instance.remove_character(character.to_map_item());
                            }
                        }
                        Event::CharacterUpdatePosition(event) => {
                            let character = characters.get_mut(&event.char_id).unwrap();
                            character.update_position(event.x, event.y);
                        }
                        Event::CharacterClearFov(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.clear_map_view();
                        }
                        Event::CharacterRemove(char_id) => {
                            characters.remove(&char_id);
                        }
                        Event::CharacterLoadedFromClientSide(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.loaded_from_client_side = true;
                        }
                    }
                }
            }
            for (_, character) in characters.iter_mut().filter(|(_, character)| character.loaded_from_client_side) {
                character.load_units_in_fov(server_ref.clone(), client_notification_sender_clone.clone())
            }
            sleep(Duration::from_millis(17));
        }
    }


}
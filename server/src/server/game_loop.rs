use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::{Scope, sleep};
use std::time::{Duration, Instant};
use packets::packets::{Packet, PacketZcNpcackMapmove};
use crate::server::core::event::Event;
use crate::server::core::map::MAP_EXT;
use crate::server::core::notification::{CharNotification, Notification};
use crate::server::server::Server;
use crate::util::string::StringUtil;

impl Server {
    pub(crate) fn game_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>) {
        loop {
            let start = Instant::now();
            if let Some(tasks) = server_ref.pop_task() {
                for task in tasks {
                    match task {
                        Event::CharacterChangeMap(event) => {
                            let mut characters = server_ref.characters.borrow_mut();
                            let character = characters.get_mut(&event.char_id).unwrap();
                            if let Some(map) = server_ref.maps.get(event.new_map_name.as_str()) {
                                character.join_and_set_map(map.get_instance(event.new_instance_id));
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
                            let mut characters = server_ref.characters.borrow_mut();
                            let character = characters.get_mut(&char_id).unwrap();
                            character.remove_from_existing_map();
                        }
                        Event::CharacterUpdatePosition(event) => {
                            let mut characters = server_ref.characters.borrow_mut();
                            let character = characters.get_mut(&event.char_id).unwrap();
                            character.update_position(event.x, event.y);
                        }
                    }
                }
            }
            sleep(Duration::from_millis(17));
        }
    }
}
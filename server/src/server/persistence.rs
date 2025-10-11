use std::sync::Arc;
use std::sync::mpsc::Receiver;

use crate::server::Server;
use crate::{Map, PersistenceEvent, Repository};

impl Server {
    pub(crate) fn persistence_thread(
        server_ref: Arc<Server>,
        persistence_event_receiver: Receiver<PersistenceEvent>,
        repository: Arc<dyn Repository>,
    ) {
        while let Ok(event) = persistence_event_receiver.recv() {
            server_ref.runtime.block_on(async {
                match event {
                    PersistenceEvent::SaveCharacterPosition(save_character_position) => {
                        repository
                            .character_save_position(
                                save_character_position.char_id,
                                Map::name_without_ext(&save_character_position.map_name),
                                save_character_position.x,
                                save_character_position.y,
                            )
                            .await
                            .unwrap();
                    }
                    PersistenceEvent::UpdateCharacterStatusU32(status_update) => {
                        repository
                            .character_update_status(status_update.char_id, status_update.db_column, status_update.value)
                            .await
                            .unwrap();
                    }
                    PersistenceEvent::DeleteItemsFromInventory(delete_items) => {
                        repository.character_inventory_delete(delete_items).await.unwrap();
                    }
                    PersistenceEvent::UpdateEquippedItems(items) => {
                        repository.character_inventory_wearable_item_update(items).await.unwrap();
                    }
                    PersistenceEvent::ResetSkills(reset_skills) => {
                        repository
                            .character_reset_skills(reset_skills.char_id, reset_skills.skills)
                            .await
                            .unwrap();
                    }
                    PersistenceEvent::IncreaseSkillLevel(increase_skill_level) => {
                        repository
                            .character_allocate_skill_point(
                                increase_skill_level.char_id,
                                increase_skill_level.skill.id() as i32,
                                increase_skill_level.increment,
                            )
                            .await
                            .unwrap();
                    }
                    PersistenceEvent::Shutdown => {
                        return;
                    }
                }
            });
            if !server_ref.is_alive() {
                break;
            }
        }
    }
}

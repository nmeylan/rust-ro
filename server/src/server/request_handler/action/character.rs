use packets::packets::PacketCzStatusChange;
use crate::server::model::events::game_event::{CharacterUpdateStat, GameEvent};
use crate::server::model::request::Request;
use crate::server::Server;

pub fn handle_player_status_change(server: &Server, context: Request) {
    let packez_cz_status_change = cast!(context.packet(), PacketCzStatusChange);
    server.tasks_queue.add_to_first_index(GameEvent::CharacterUpdateStat(CharacterUpdateStat{
        char_id: context.session().char_id.unwrap(),
        stat_id: packez_cz_status_change.status_id,
        change_amount: packez_cz_status_change.change_amount as u16,
    }));
}
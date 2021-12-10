use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use packets::packets::{PacketCzPlayerChat, PacketZcNotifyPlayerchat, PacketZcNpcackMapmove};
use crate::Server;
use crate::server::core::session::Session;
use regex::Regex;
use std::io::Write;
use packets::packets::Packet;
use crate::server::core::character_movement::change_map;

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"^([@#!])([^\s]*)\s?(.*)?").unwrap();
}
pub fn handle_atcommand(server: Arc<Server>, packet: &PacketCzPlayerChat, _runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, session: Arc<Session>) {
    let index_of_colon = packet.msg.find(":").unwrap();
    let command_txt = &packet.msg[index_of_colon + 1..].trim();
    let maybe_captures = COMMAND_REGEX.captures(command_txt);
    if maybe_captures.is_none() {
        return;
    }
    let captures = maybe_captures.unwrap();
    if captures.len() < 2 {
        return;
    }
    let symbol = captures.get(1).unwrap().as_str();
    let command = captures.get(2).unwrap().as_str();
    let mut args = "";
    if captures.len() > 2 {
        args = captures.get(3).unwrap().as_str();
    }
    let mut packet_zc_notify_playerchat = PacketZcNotifyPlayerchat::new();
    // let mut packets = vec![];
    match command {
        "go" => {
            handle_go(args, session);
            packet_zc_notify_playerchat.set_msg(format!("Warp"));

        }
        _ => {
            packet_zc_notify_playerchat.set_msg(format!("{}{} is an Unknown Command.", symbol, command));
        }
    }
    packet_zc_notify_playerchat.set_packet_length((4 + packet_zc_notify_playerchat.msg.len()) as i16);
    packet_zc_notify_playerchat.fill_raw();
    socket_send!(tcp_stream, packet_zc_notify_playerchat.raw());
}

pub fn handle_go(args: &str, session: Arc<Session>) {
    change_map("prontera".to_string(), 156, 191, session.clone(), session.get_character());

}
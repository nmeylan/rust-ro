use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use packets::packets::{PacketCzPlayerChat, PacketZcNotifyPlayerchat};
use crate::{Map, Server};
use crate::server::core::session::Session;
use regex::Regex;
use std::io::Write;
use std::sync::atomic::Ordering::Relaxed;
use packets::packets::Packet;
use crate::server::configuration::CityConfig;
use crate::server::core::character_movement::change_map;
use crate::server::core::map::RANDOM_CELL;

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"^([@#!])([^\s]*)\s?(.*)?").unwrap();
}
pub fn handle_atcommand(server: Arc<Server>, packet: &PacketCzPlayerChat, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, session: Arc<Session>) {
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
    let mut args = Vec::<&str>::new();
    if captures.len() > 2 {
        args = captures.get(3).unwrap().as_str().split(" ").collect();
    }
    let mut packet_zc_notify_playerchat = PacketZcNotifyPlayerchat::new();
    // let mut packets = vec![];
    match command {
        "go" => {
            let result = handle_go(server, session, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "warp"|"rura"|"warpto" => {
            let result = handle_warp(server, session, args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        _ => {
            packet_zc_notify_playerchat.set_msg(format!("{}{} is an Unknown Command.", symbol, command));
        }
    }
    packet_zc_notify_playerchat.set_packet_length((4 + packet_zc_notify_playerchat.msg.len()) as i16);
    packet_zc_notify_playerchat.fill_raw();
    socket_send!(tcp_stream, packet_zc_notify_playerchat.raw());
}

pub fn handle_go(server: Arc<Server>, session: Arc<Session>, args: Vec::<&str>) -> String {
    let cities_len = server.configuration.maps.cities.len();
    let cleaned_arg = args[0].trim();
    let mut maybe_city: Option<&CityConfig> = None;
    match cleaned_arg.parse::<i8>() {
        Ok(index) => {
            if index < cities_len as i8 {
                maybe_city = unsafe { Some(server.configuration.maps.cities.get_unchecked(index as usize)) } // it safe, bounds are checked
            }
        },
        _ => ()
    }
    if maybe_city.is_none() {
        // aliases
        let name = match cleaned_arg {
            "old_moc" => "morroc".to_string(),
            "morocc" => "morroc".to_string(),
            "lutie" => "xmas".to_string(),
            "juno" => "yuno".to_string(),
            "kunlun" => "gornyun".to_string(),
            "luoyang" => "louyang".to_string(),
            "new1-1" => "novice".to_string(),
            "startpoint" => "novice".to_string(),
            "beginning" => "novice".to_string(),
            "prison" => "jail".to_string(),
            "sec_pri" => "jail".to_string(),
            "rael" => "rachel".to_string(),
            _ => cleaned_arg.to_string()
        };
        maybe_city = server.configuration.maps.cities.iter().find(|city| {
            city.name == name

        });
    }
    if maybe_city.is_none() {
        return format!("Can't find map by index or name with given argument: {}", cleaned_arg);
    }
    let mut city = maybe_city.unwrap().clone();

    match city.name.as_str() {
        // To match client side name
        "morroc" => city.name = "old_moc".to_string(),
        "lutie" => city.name = "xmas".to_string(),
        "juno" => city.name = "yuno".to_string(),
        "kunlun" => city.name = "gornyun".to_string(),
        "luoyang" => city.name = "louyang".to_string(),
        "novice" => city.name = "new1-1".to_string(),
        "startpoint" => city.name = "new1-1".to_string(),
        "beginning" => city.name = "new1-1".to_string(),
        "prison" => city.name = "sec_pri".to_string(),
        "jail" => city.name = "sec_pri".to_string(),
        "rael" => city.name = "rachel".to_string(),
        _ => ()
    }

    change_map(city.name.clone(), city.x, city.y, session.clone(), server);
    format!("Warping at {} {},{}", city.name.clone(), city.x, city.y)
}

pub fn handle_warp(server: Arc<Server>, session: Arc<Session>, args: Vec::<&str>) -> String {
    let map_name = args[0].to_string();
    if server.maps.contains_key(&map_name) {
        change_map(map_name.clone(), RANDOM_CELL.0, RANDOM_CELL.1, session.clone(), server);
        let char_session = session.character.as_ref().unwrap();
        return format!("Warp to map {} at {},{}", map_name, char_session.get_x(), char_session.get_y());
    }
    return format!("Map not found: {}", map_name);
}
use std::sync::Arc;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use packets::packets::{PacketCzPlayerChat, PacketZcNotifyPlayerchat};
use crate::server::model::session::Session;
use models::enums::class::JobName;
use models::enums::EnumWithStringValue;
use models::enums::EnumWithNumberValue;
use std::fmt::Write;
use std::time::Instant;
use regex_lite::Regex;

use packets::packets::Packet;
use crate::load_scripts;
use configuration::configuration::CityConfig;
use crate::server::model::map::RANDOM_CELL;
use crate::server::model::request::Request;
use crate::server::model::events::game_event::{CharacterChangeJob, CharacterChangeJobLevel, CharacterChangeLevel, GameEvent};
use crate::server::script::Value;
use crate::server::Server;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_service::ScriptService;
use crate::server::service::server_service::ServerService;



lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"^([@#!])([^\s]*)\s?(.*)?").unwrap();
}
pub fn handle_atcommand(server: &Server, context: Request, packet: &PacketCzPlayerChat) {
    let index_of_colon = packet.msg.find(':').unwrap();
    let command_txt = &packet.msg[index_of_colon + 1..packet.msg.len() - 1].trim();
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
        args = captures.get(3).unwrap().as_str().split(' ').map(|arg| arg.trim_matches(char::from(0))).collect();
    }
    let mut packet_zc_notify_playerchat = PacketZcNotifyPlayerchat::new(GlobalConfigService::instance().packetver());
    // let mut packets = vec![];
    match command {
        "go" => {
            debug!("{:?}", args);
            let result = handle_go(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "warp" | "rura" | "warpto" => {
            let result = handle_warp(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "item" => {
            let result = handle_item(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "inspect" | "i" => {
            let result = handle_inspect(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "blvl" | "lvup" | "blevel" | "baselvl" | "baselvup" | "baselevel" | "baselvlup" => {
            let result = handle_base_level(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "setblvl" | "setblevel" | "setbaselvl" | "setbaselevel" => {
            let result = handle_set_base_level(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "jlvl" | "jlvup" | "jlevel" | "joblvl" | "joblvup" | "joblevel" | "joblvlup" => {
            let result = handle_job_level(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "setjlvl" | "setjlevel" | "setjoblvl" | "setjoblevel" => {
            let result = handle_set_job_level(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "job" | "jobchange" => {
            let result = handle_set_job(server, context.session().char_id(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "rates" => {
            let result = handle_rates(server);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "reload" => {
            let result = handle_reload(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "resetskills" => {
            let result = handle_reset_skills(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "resetstats" => {
            let result = handle_reset_stats(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "speed" => {
            let result = handle_speed_change(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        "heal" => {
            let result = handle_heal(server, context.session(), context.runtime(), args);
            packet_zc_notify_playerchat.set_msg(result);
        }
        _ => {
            packet_zc_notify_playerchat.set_msg(format!("{symbol}{command} is an Unknown Command."));
        }
    }
    packet_zc_notify_playerchat.set_packet_length((4 + packet_zc_notify_playerchat.msg.len()) as i16);
    packet_zc_notify_playerchat.fill_raw();
    socket_send!(context, packet_zc_notify_playerchat);
}

pub fn handle_go(server: &Server, session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    let cities_len = server.configuration.maps.cities.len();
    let cleaned_arg = args[0].trim();
    let mut maybe_city: Option<&CityConfig> = None;
    if let Ok(index) = cleaned_arg.parse::<i8>() {
        if index < cities_len as i8 {
            maybe_city = unsafe { Some(server.configuration.maps.cities.get_unchecked(index as usize)) } // it safe, bounds are checked
        }
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
        return format!("Can't find map by index or name with given argument: {cleaned_arg}");
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

    server.server_service().schedule_warp_to_walkable_cell(server.state_mut().as_mut(),&city.name, city.x, city.y, session.char_id());
    format!("Warping at {} {},{}", city.name.clone(), city.x, city.y)
}

pub fn handle_warp(server: &Server, session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    let map_name = args[0].to_string();
    if GlobalConfigService::instance().maps().contains_key(&map_name) {
        let mut x = RANDOM_CELL.0;
        let mut y = RANDOM_CELL.1;
        if args.len() > 2 {
            let parse_x_res = args[1].parse::<u16>();
            let parse_y_res = args[2].parse::<u16>();
            if let Ok(parse_x_res) = parse_x_res {
                x = parse_x_res;
            }
            if let Ok(parse_y_res) = parse_y_res {
                y = parse_y_res;
            }
        }
        server.server_service().schedule_warp_to_walkable_cell(server.state_mut().as_mut(), &map_name, x, y, session.char_id());
        let char_id = session.char_id();
        let character = server.state().get_character_unsafe(char_id);
        return format!("Warp to map {} at {},{}", map_name, character.x(), character.y());
    }
    format!("Map not found: {map_name}")
}

pub fn handle_item(server: &Server, session: Arc<Session>, runtime: &Runtime, args: Vec::<&str>) -> String {
    if args.is_empty() {
        return format!("@item command accept from 1 to 2 parameters but received {}", args.len());
    }
    server.script_service().schedule_get_items(session.char_id(), runtime, vec![
        (args[0].parse::<i32>().map(Value::Number).unwrap_or(Value::String(args[0].to_string())),
         args.get(1).unwrap_or(&"1").parse::<i16>().unwrap_or(1))], false);

    String::new()
}

pub fn handle_inspect(server: &Server, session: Arc<Session>, _runtime: &Runtime, _args: Vec::<&str>) -> String {
    let char_id = session.char_id();
    let character = server.state().get_character_unsafe(char_id);
    server.character_service().print(character);
    String::new()
}

pub fn handle_base_level(server: &Server, session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    if args.is_empty() {
        return "@baselevel command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterChangeLevel(CharacterChangeLevel { char_id: session.char_id(), set_level: None, add_level: Some(args.first().unwrap().parse::<i32>().unwrap_or(0)) }));
    String::new()
}

pub fn handle_set_base_level(server: &Server, session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    if args.is_empty() {
        return "@set_baselevel command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterChangeLevel(CharacterChangeLevel {
        char_id: session.char_id(),
        set_level: args.first().unwrap().parse::<u32>().ok(),
        add_level: None,
    }));
    String::new()
}

pub fn handle_job_level(server: &Server, session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    if args.is_empty() {
        return "@joblevel command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterChangeJobLevel(CharacterChangeJobLevel { char_id: session.char_id(), set_level: None, add_level: Some(args.first().unwrap().parse::<i32>().unwrap_or(0)) }));
    String::new()
}

pub fn handle_set_job_level(server: &Server, session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    if args.is_empty() {
        return "@set_joblevel command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterChangeJobLevel(CharacterChangeJobLevel {
        char_id: session.char_id(),
        set_level: args.first().unwrap().parse::<u32>().ok(),
        add_level: None,
    }));
    String::new()
}

pub fn handle_set_job(server: &Server, char_id: u32, args: Vec::<&str>) -> String {
    if args.is_empty() {
        return "@job command accept 1 parameters but received none".to_string();
    }
    let maybe_job = if let Ok(job_id) = args.first().unwrap().parse::<u32>() {
        JobName::try_from_value(job_id as usize)
    } else {
        JobName::try_from_string(args.first().unwrap())
    };
    if let Ok(job) = maybe_job {
        server.add_to_next_tick(GameEvent::CharacterChangeJob(CharacterChangeJob { char_id, job, should_reset_skills: true }));
        return "Your job has been changed.".to_string();
    }
    format!("Job {} not found", args.first().unwrap())
}

pub fn handle_rates(server: &Server) -> String {
    let mut msg = String::new();
    writeln!(msg, "Experience rates: Base {:.2}x / Job {:.2}", server.configuration.game.base_exp_rate,server.configuration.game.job_exp_rate).unwrap();
    writeln!(msg, "Normal Drop Rates: Common {:.2}x / Healing {:.2}x / Usable {:.2}x Equipment {:.2}x / Card {:.2}x", server.configuration.game.drop_rate, server.configuration.game.drop_rate, server.configuration.game.drop_rate, server.configuration.game.drop_rate, server.configuration.game.drop_rate_card).unwrap();
    writeln!(msg, "Boss  Drop Rates: Common {:.2}x / Healing {:.2}x / Usable {:.2}x Equipment {:.2}x / Card {:.2}x", server.configuration.game.drop_rate_mvp, server.configuration.game.drop_rate_mvp, server.configuration.game.drop_rate_mvp, server.configuration.game.drop_rate_mvp, server.configuration.game.drop_rate_card ).unwrap();
    msg
}

pub fn handle_reload(server: &Server, _session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    // TODO check if user privileges
    if args.is_empty() {
        return "@reload command accept 1 parameters but received none".to_string();
    }
    match args[0] {
        "script" => {
            let start = Instant::now();
            let scripts = load_scripts(server.script_service().vm.clone());
            format!("{} scripts have been recompiled and reloaded in {} secs", scripts.len(), start.elapsed().as_millis() as f32 / 1000.0)
        }
        &_ => format!("@reload command accept a string value among: [script] but received {}", args[0])
    }
}

pub fn handle_reset_skills(server: &Server, session: Arc<Session>, _runtime: &Runtime, _args: Vec::<&str>) -> String {
    server.add_to_next_tick(GameEvent::CharacterResetSkills(session.char_id()));
    "Skills have been reset.".to_string()
}


pub fn handle_reset_stats(server: &Server, session: Arc<Session>, _runtime: &Runtime, _args: Vec::<&str>) -> String {
    server.add_to_next_tick(GameEvent::CharacterResetStats(session.char_id()));
    "Stats have been reset.".to_string()
}


pub fn handle_speed_change(server: &Server, session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    if args.is_empty() {
        return "@speed command accept 1 parameters but received none".to_string();
    }
    let mut speed = args[0].parse::<u16>().unwrap_or(150_u16);
    if speed < 50 {
        speed = 50;
    } else if speed > 500 {
        speed = 500;
    }
    server.add_to_next_tick(GameEvent::CharacterUpdateSpeed(session.char_id(), speed));
    format!("Speed has been set at {}.", speed)
}


pub fn handle_heal(server: &Server, session: Arc<Session>, _runtime: &Runtime, args: Vec::<&str>) -> String {
    if args.is_empty() {
        return "@speed command accept 1 parameters but received none".to_string();
    }
    server.add_to_next_tick(GameEvent::CharacterRestoreAllHpAndSP(session.char_id()));
    "Restored all HP and SP".to_string()
}

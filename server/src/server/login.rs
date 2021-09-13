use crate::packets::packets::{PacketCaLogin, PacketAcAcceptLogin2, Packet, ServerAddr2, PacketAcRefuseLoginR3};
use crate::repository::lib::Repository;
use sqlx::{MySql, Row};
use rand::Rng;
use tokio::runtime::Runtime;
use crate::server::core::{FeatureState, Session, Server};
use std::net::TcpStream;
use std::sync::{Mutex, Arc};
use std::io::Write;

pub(crate) fn handle_login(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>) -> FeatureState {
    let res = runtime.block_on(async {
        authenticate(packet.as_any().downcast_ref::<PacketCaLogin>().unwrap(), &server.repository).await
    });
    if res.as_any().downcast_ref::<PacketAcAcceptLogin2>().is_some() {
        let packet_response = res.as_any().downcast_ref::<PacketAcAcceptLogin2>().unwrap();
        // Currently only handle this account to be able to still use proxy in other accounts
        if packet_response.aid != 2000000 {
            return FeatureState::Unimplemented;
        }
        let new_user_session = Session {
            char_server_socket: None,
            map_server_socket: None,
            account_id: packet_response.aid,
            login_id1: packet_response.auth_code,
            login_id2: packet_response.user_level
        };
        let mut server_context_guard = server.server_context.lock().unwrap();
        server_context_guard.sessions.insert(packet_response.aid.clone(), new_user_session);

        let mut tcp_stream_guard = tcp_stream.lock().unwrap();
        tcp_stream_guard.write(res.raw());
        tcp_stream_guard.flush();
        std::mem::drop(tcp_stream_guard);
        return FeatureState::Implemented(res);
    } else if res.as_any().downcast_ref::<PacketAcRefuseLoginR3>().is_some() {
        let mut tcp_stream_guard = tcp_stream.lock().unwrap();
        tcp_stream_guard.write(res.raw());
        tcp_stream_guard.flush();
        return FeatureState::Implemented(res);
    }
    FeatureState::Unimplemented
}

pub async fn authenticate(packet: &PacketCaLogin, repository: &Repository<MySql>) -> Box<dyn Packet> {
    let mut rng = rand::thread_rng();
    let mut username = String::new();
    let mut password = String::new();
    for c in packet.id {
        if c == 0 as char { break; } else { username.push(c) }
    }
    for c in packet.passwd {
        if c == 0 as char { break; } else { password.push(c) }
    }
    let mut row_result = sqlx::query("SELECT * FROM login WHERE userid = ? AND user_pass = ?") // TODO add bcrypt on user_pass column, but not supported by hercules
        .bind(username)
        .bind(password)
        .fetch_one(&repository.pool).await;
    if row_result.is_ok() {
        let row = row_result.unwrap();
        let account_id: u32 = row.get("account_id");
        // TODO check credentials with database entries
        let mut ac_accept_login2 = PacketAcAcceptLogin2::new();
        ac_accept_login2.set_packet_length(224);
        ac_accept_login2.set_aid(account_id);
        ac_accept_login2.set_auth_code(rng.gen::<i32>());
        ac_accept_login2.set_user_level(rng.gen::<u32>());
        ac_accept_login2.set_sex(1);
        let mut server = ServerAddr2::new();
        server.set_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        server.set_port(6123);
        let mut name_chars = [0 as char; 20];
        "Rust ragnarok".chars().enumerate().for_each(|(i, c)| name_chars[i] = c);
        server.set_name(name_chars);
        ac_accept_login2.set_server_list(vec![server]);
        ac_accept_login2.fill_raw();
        return Box::new(ac_accept_login2)
    }
    let mut r2 = PacketAcRefuseLoginR3::new();
    r2.set_error_code(1);
    r2.fill_raw();
    return Box::new(r2)
}
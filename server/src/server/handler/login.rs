use crate::packets::packets::{PacketCaLogin, PacketAcAcceptLogin2, Packet, ServerAddr2, PacketAcRefuseLoginR3};
use crate::repository::lib::Repository;
use sqlx::{MySql, Row};
use rand::Rng;
use tokio::runtime::Runtime;
use std::net::{TcpStream, Shutdown};
use std::sync::{Arc, RwLock};
use std::io::{Write, Read};

use std::thread::spawn;
use crate::packets::packets_parser::parse;
use crate::server::core::session::Session;
use crate::server::server::Server;

pub(crate) fn handle_login(server: Arc<Server>, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>) {
    let res = runtime.block_on(async {
        authenticate(cast!(packet, PacketCaLogin), &server.repository).await
    });
    if res.as_any().downcast_ref::<PacketAcAcceptLogin2>().is_some() {
        let packet_response = res.as_any().downcast_ref::<PacketAcAcceptLogin2>().unwrap();
        // Currently only handle this account to be able to still use proxy in other accounts
        if packet_response.aid != 2000000 {
            proxy_login(packet, tcp_stream);
            return;
        }
        let new_user_session = Session {
            char_server_socket: None,
            map_server_socket: None,
            account_id: packet_response.aid,
            auth_code: packet_response.auth_code,
            user_level: packet_response.user_level,
            character: None
        };
        {
            let mut sessions_guard = server.sessions.write().unwrap();
            sessions_guard.insert(packet_response.aid.clone(), Arc::new(RwLock::new(new_user_session)));
        }
        socket_send!(tcp_stream, res.raw());
    } else if res.as_any().downcast_ref::<PacketAcRefuseLoginR3>().is_some() {
        socket_send!(tcp_stream, res.raw());
    }
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
    let row_result = sqlx::query("SELECT * FROM login WHERE userid = ? AND user_pass = ?") // TODO add bcrypt on user_pass column, but not supported by hercules
        .bind(username)
        .bind(password)
        .fetch_one(&repository.pool).await;
    if row_result.is_ok() {
        let row = row_result.unwrap();
        let account_id: u32 = row.get("account_id");
        let mut ac_accept_login2 = PacketAcAcceptLogin2::new();
        ac_accept_login2.set_packet_length(224);
        ac_accept_login2.set_aid(account_id);
        ac_accept_login2.set_auth_code(rng.gen::<i32>());
        ac_accept_login2.set_user_level(rng.gen::<u32>());
        ac_accept_login2.set_sex(1);
        let mut server = ServerAddr2::new();
        server.set_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        server.set_port(6901);
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

fn proxy_login(packet: &mut dyn Packet, tcp_stream: Arc<RwLock<TcpStream>>) {
    let target = format!("127.0.0.1:6900");
    let mut remote_login_server = TcpStream::connect(target.clone()).map_err(|error| format!("Could not establish connection to {}: {}", target, error)).unwrap();
    let mut remote_login_server_clone = remote_login_server.try_clone().unwrap();
    let handle = spawn(move || {
        let mut buffer = [0; 2048];
        loop {
            match remote_login_server_clone.read(&mut buffer) {
                Ok(bytes_read) => {
                    // no more data
                    if bytes_read == 0 {
                        remote_login_server_clone.shutdown(Shutdown::Both);
                        break;
                    }
                    let mut packet = parse(&mut buffer[..bytes_read]);
                    if packet.as_any().downcast_ref::<PacketAcAcceptLogin2>().is_some() {
                        let packet_accept_login2 = packet.as_any_mut().downcast_mut::<PacketAcAcceptLogin2>().unwrap();
                        let server_char = packet_accept_login2.server_list.get_mut(0).unwrap();
                        server_char.set_port(6123);
                        packet_accept_login2.fill_raw();
                        let mut tcp_stream_guard = tcp_stream.write().unwrap();
                        tcp_stream_guard.write(packet_accept_login2.raw());
                        tcp_stream_guard.flush();
                    } else {
                        panic!("Received packet is not PacketAcAcceptLogin2");
                    }
                }
                _ => {}
            }
        }
    });
    remote_login_server.write(packet.raw());
    remote_login_server.flush();
    handle.join();
}
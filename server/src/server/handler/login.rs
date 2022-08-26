use std::io::{Read, Write};
use std::mem;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread::spawn;

use rand::Rng;
use sqlx::Row;

use packets::packets::{Packet, PacketAcAcceptLogin, PacketAcAcceptLogin2, PacketAcRefuseLogin, PacketAcRefuseLoginR2, PacketAcRefuseLoginR3, PacketCaLogin, ServerAddr, ServerAddr2};
use packets::packets_parser::parse;

use crate::repository::Repository;
use crate::server::core::request::Request;
use crate::server::core::response::Response;
use crate::server::core::session::Session;
use crate::server::server::Server;

pub(crate) fn handle_login(server: Arc<Server>, context: Request) {
    let packet_ca_login = cast!(context.packet(), PacketCaLogin);
    let mut res = context.runtime().block_on(async {
        authenticate(server.clone(), packet_ca_login, &server.repository).await
    });
    info!("packetver {}", packet_ca_login.version);
    if res.as_any().downcast_ref::<PacketAcAcceptLogin2>().is_some() {
        let packet_response = res.as_any().downcast_ref::<PacketAcAcceptLogin2>().unwrap();
        // Currently only handle this account to be able to still use proxy in other accounts
        if !server.configuration.server.accounts.contains(&packet_response.aid) {
            proxy_login(server.clone(), context.packet(), context.socket());
            return;
        }
        let new_user_session = Session::create_empty(packet_response.aid,packet_response.auth_code,packet_response.user_level, packet_ca_login.version); // TODO: packetver find solution to allow client to set packetver
        let mut sessions_guard = write_lock!(server.sessions);
        sessions_guard.insert(packet_response.aid, Arc::new(new_user_session));
        socket_send!(context, res);
    } else if res.as_any().downcast_ref::<PacketAcAcceptLogin>().is_some() {
        let packet_response = res.as_any().downcast_ref::<PacketAcAcceptLogin>().unwrap();
        // Currently only handle this account to be able to still use proxy in other accounts
        if !server.configuration.server.accounts.contains(&packet_response.aid) {
            proxy_login(server.clone(), context.packet(), context.socket());
            return;
        }
        let new_user_session = Session::create_empty(packet_response.aid,packet_response.auth_code,packet_response.user_level, packet_ca_login.version); // TODO: packetver find solution to allow client to set packetver
        let mut sessions_guard = write_lock!(server.sessions);
        sessions_guard.insert(packet_response.aid, Arc::new(new_user_session));
        socket_send!(context, res);
    } else {
        socket_send!(context, res);
    }
}

pub async fn authenticate(server: Arc<Server>, packet: &PacketCaLogin, repository: &Repository) -> Box<dyn Packet> {
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
        if server.packetver() < 20170315 {
            let mut ac_accept_login = PacketAcAcceptLogin::new();
            ac_accept_login.set_packet_length(PacketAcAcceptLogin::base_len(server.packetver()) as i16);
            ac_accept_login.set_aid(account_id);
            ac_accept_login.set_auth_code(rng.gen::<i32>());
            ac_accept_login.set_user_level(rng.gen::<u32>());
            ac_accept_login.set_sex(1);
            let mut server_addr = ServerAddr::new();
            server_addr.set_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
            server_addr.set_port(server.configuration.server.port as i16);
            let mut name_chars = [0 as char; 20];
            "Rust ragnarok".chars().enumerate().for_each(|(i, c)| name_chars[i] = c);
            server_addr.set_name(name_chars);
            ac_accept_login.set_server_list(vec![server_addr]);
            ac_accept_login.fill_raw();
            return Box::new(ac_accept_login)
        } else {
            let mut ac_accept_login2 = PacketAcAcceptLogin2::new();
            ac_accept_login2.set_packet_length(PacketAcAcceptLogin2::base_len(server.packetver()) as i16);
            ac_accept_login2.set_aid(account_id);
            ac_accept_login2.set_auth_code(rng.gen::<i32>());
            ac_accept_login2.set_user_level(rng.gen::<u32>());
            ac_accept_login2.set_sex(1);
            let mut server_addr = ServerAddr2::new();
            server_addr.set_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
            server_addr.set_port(server.configuration.server.port as i16);
            let mut name_chars = [0 as char; 20];
            "Rust ragnarok".chars().enumerate().for_each(|(i, c)| name_chars[i] = c);
            server_addr.set_name(name_chars);
            ac_accept_login2.set_server_list(vec![server_addr]);
            ac_accept_login2.fill_raw();
            return Box::new(ac_accept_login2)
        }

    }
    let refuse_login_packet: Box<dyn Packet>;
    if server.packetver() >= 20180627 {
        let mut refuse_login_packet3 = PacketAcRefuseLoginR3::new();
        refuse_login_packet3.set_error_code(1);
        refuse_login_packet3.fill_raw();
        refuse_login_packet = Box::new(refuse_login_packet3);
    } else if server.packetver() > 20101123 {
        let mut refuse_login_packet2 = PacketAcRefuseLoginR2::new();
        refuse_login_packet2.set_error_code(1);
        refuse_login_packet2.fill_raw();
        refuse_login_packet = Box::new(refuse_login_packet2);
    } else {
        let mut refuse_login_packet1 = PacketAcRefuseLogin::new();
        refuse_login_packet1.set_error_code(1);
        refuse_login_packet1.fill_raw();
        refuse_login_packet = Box::new(refuse_login_packet1);
    }
    refuse_login_packet
}

fn proxy_login(server: Arc<Server>, packet: &dyn Packet, tcp_stream: Arc<RwLock<TcpStream>>) {
    let target = format!("{}:{}", server.configuration.proxy.remote_login_server_ip, server.configuration.proxy.remote_login_server_port);
    let mut remote_login_server = TcpStream::connect(target.clone()).map_err(|error| format!("Could not establish connection to {}: {}", target, error)).unwrap();
    let mut remote_login_server_clone = remote_login_server.try_clone().unwrap();
    let handle = spawn(move || {
        let mut buffer = [0; 2048];
        loop {
            match remote_login_server_clone.read(&mut buffer) {
                Ok(bytes_read) => {
                    // no more data
                    if bytes_read == 0 {
                        remote_login_server_clone.shutdown(Shutdown::Both).expect("Unable to shutdown remote login server socket");
                        break;
                    }
                    let mut packet = parse(&buffer[..bytes_read], server.packetver());
                    if packet.as_any().downcast_ref::<PacketAcAcceptLogin2>().is_some() {
                        let packet_accept_login2 = packet.as_any_mut().downcast_mut::<PacketAcAcceptLogin2>().unwrap();
                        let server_char = packet_accept_login2.server_list.get_mut(0).unwrap();
                        server_char.set_port(server.configuration.proxy.local_char_server_port as i16);
                        packet_accept_login2.fill_raw();
                        let mut tcp_stream_guard = tcp_stream.write().unwrap();
                        debug!("Respond with: {:02X?}", packet_accept_login2.raw());
                        tcp_stream_guard.write_all(packet_accept_login2.raw()).unwrap();
                        tcp_stream_guard.flush().unwrap();
                    } else if packet.as_any().downcast_ref::<PacketAcAcceptLogin>().is_some() {
                        let packet_accept_login = packet.as_any_mut().downcast_mut::<PacketAcAcceptLogin>().unwrap();
                        let server_char = packet_accept_login.server_list.get_mut(0).unwrap();
                        server_char.set_port(server.configuration.proxy.local_char_server_port as i16);
                        packet_accept_login.fill_raw();
                        let mut tcp_stream_guard = tcp_stream.write().unwrap();
                        debug!("Respond with: {:02X?}", packet_accept_login.raw());
                        tcp_stream_guard.write_all(packet_accept_login.raw()).unwrap();
                        tcp_stream_guard.flush().unwrap();
                    } else {
                        panic!("Received packet is not PacketAcAcceptLogin2");
                    }
                }
                _ => {}
            }
        }
    });
    remote_login_server.write_all(packet.raw()).expect("Failed to write packet for remote login server");
    remote_login_server.flush().expect("Failed to flush packet for remote login server");
    handle.join().expect("Failed to await login request proxy");
}
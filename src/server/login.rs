use crate::packets::packets::{PacketCaLogin, PacketAcAcceptLogin2, Packet, ServerAddr2};
use crate::repository::lib::Repository;
use sqlx::MySql;
use rand::Rng;

pub fn authenticate(packet: &PacketCaLogin, pool: &Repository<MySql>) -> Box<dyn Packet> {
    let mut rng = rand::thread_rng();
    // TODO check credentials with database entries
    let mut ac_accept_login2 = PacketAcAcceptLogin2::new();
    ac_accept_login2.set_packet_length(224);
    ac_accept_login2.set_aid(2000000);
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
    Box::new(ac_accept_login2)
}
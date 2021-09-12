use crate::packets::packets::{PacketCaLogin, PacketAcAcceptLogin2, Packet, ServerAddr2, PacketAcRefuseLoginR3};
use crate::repository::lib::Repository;
use sqlx::{MySql, Row};
use rand::Rng;

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
        println!("{:?}", account_id);
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
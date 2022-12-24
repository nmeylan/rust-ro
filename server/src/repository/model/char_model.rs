use sqlx::{FromRow, Error, Row};
use packets::packets::CharacterInfoNeoUnion;
use sqlx::postgres::{PgRow};

pub struct CharacterInfoNeoUnionWrapped {
    pub data: CharacterInfoNeoUnion,
}

impl<'r> FromRow<'r, PgRow> for CharacterInfoNeoUnionWrapped {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let mut character_info_neo_union = CharacterInfoNeoUnion::new();

        character_info_neo_union.set_gid(row.get::<i32, _>("char_id") as u32);
        character_info_neo_union.set_exp(row.get::<i32, _>("base_exp") as u32);
        character_info_neo_union.set_exp_64(row.get::<i32, _>("base_exp") as u64);
        character_info_neo_union.set_money(row.get::<i32, _>("zeny") as u32);
        character_info_neo_union.set_jobexp(row.get::<i32, _>("job_exp") as u32);
        character_info_neo_union.set_jobexp_64(row.get::<i32, _>("job_exp") as u64);
        character_info_neo_union.set_joblevel(row.get::<i32, _>("job_level") as u32);
        character_info_neo_union.set_bodystate(0_u32);
        character_info_neo_union.set_healthstate(0_u32);
        character_info_neo_union.set_effectstate(row.get::<i32, _>("option"));
        character_info_neo_union.set_virtue(row.get::<i32, _>("karma") as i32);
        character_info_neo_union.set_honor(row.get::<i32, _>("manner") as i32);
        character_info_neo_union.set_status_point(row.get::<i16, _>("status_point") as u16);
        character_info_neo_union.set_hp(row.get::<i32, _>("hp") as u32);
        character_info_neo_union.set_hp_16(row.get::<i32, _>("hp") as u16);
        character_info_neo_union.set_maxhp(row.get::<i32, _>("max_hp") as u32);
        character_info_neo_union.set_maxhp_16(row.get::<i32, _>("max_hp") as u16);
        character_info_neo_union.set_sp(row.get::<i32, _>("sp") as u16);
        character_info_neo_union.set_maxsp(row.get::<i32, _>("max_sp") as u16);
        character_info_neo_union.set_speed(100_u16); // TODO make this configurable SPEED
        character_info_neo_union.set_class(row.get::<i16, _>("class") as u16);
        character_info_neo_union.set_head(row.get::<i16, _>("hair") as u16);
        character_info_neo_union.set_body(row.get::<i16, _>("body") as u16);
        character_info_neo_union.set_weapon(row.get::<i16, _>("weapon") as u16);
        character_info_neo_union.set_level(row.get::<i16, _>("base_level") as u16);
        character_info_neo_union.set_skill_point(row.get::<i16, _>("skill_point") as u16);
        character_info_neo_union.set_head_bottom(row.get::<i16, _>("head_bottom") as u16);
        character_info_neo_union.set_shield(row.get::<i16, _>("shield") as u16);
        character_info_neo_union.set_head_top(row.get::<i16, _>("head_top") as u16);
        character_info_neo_union.set_head_mid(row.get::<i16, _>("head_mid") as u16);
        character_info_neo_union.set_hair_color(row.get::<i16, _>("hair_color") as u16);
        character_info_neo_union.set_body_color(row.get::<i16, _>("clothes_color") as u16);
        let name: String = row.get("name");
        let mut name_as_array = [0 as char; 24];
        for (i, c) in name.chars().enumerate() {
            name_as_array[i] = c;
        }
        character_info_neo_union.set_name(name_as_array);
        character_info_neo_union.set_str(row.get::<i16, _>("str") as u8);
        character_info_neo_union.set_agi(row.get::<i16, _>("agi") as u8);
        character_info_neo_union.set_vit(row.get::<i16, _>("vit") as u8);
        character_info_neo_union.set_int(row.get::<i16, _>("int") as u8);
        character_info_neo_union.set_dex(row.get::<i16, _>("dex") as u8);
        character_info_neo_union.set_luk(row.get::<i16, _>("luk") as u8);
        character_info_neo_union.set_char_num(row.get::<i16, _>("char_num") as i8);
        character_info_neo_union.set_b_is_changed_char_name(row.get::<i16, _>("rename") as u16);
        let mut last_map: String = row.get("last_map");
        if last_map.is_empty() {
            last_map = "prontera".to_string();
        }
        last_map += ".gat";
        let mut last_map_as_array = [0 as char; 16];
        for (i, c) in last_map.chars().enumerate() {
            last_map_as_array[i] = c;
        }
        character_info_neo_union.set_last_map(last_map_as_array);
        character_info_neo_union.set_delete_date(row.get::<i32, _>("delete_date") as u32);
        character_info_neo_union.set_robe(row.get::<i32, _>("robe") as u32);
        character_info_neo_union.set_slot_addon(0);
        character_info_neo_union.set_rename_addon(0);
        character_info_neo_union.set_sex(if row.get::<String, _>("sex") == "M" { 1 } else { 0 });
        character_info_neo_union.fill_raw();
        let wrapped = CharacterInfoNeoUnionWrapped { data: character_info_neo_union };
        Ok(wrapped)
    }
}

#[derive(sql::SqlInsert)]
pub struct CharInsertModel {
    pub account_id: i32,
    pub char_num: i8,
    pub name: String,
    pub class: i16,
    pub zeny: i32,
    pub status_point: i16,
    pub str: i16,
    pub agi: i16,
    pub vit: i16,
    pub int: i16,
    pub dex: i16,
    pub luk: i16,
    pub max_hp: i32,
    pub hp: i32,
    pub max_sp: i32,
    pub sp: i32,
    pub hair: i16,
    pub hair_color: i32,
    pub last_map: String,
    pub last_x: i16,
    pub last_y: i16,
    pub save_map: String,
    pub save_x: i16,
    pub save_y: i16,
    pub sex: String,
    pub inventory_size: i32,
}

#[derive(sqlx::FromRow)]
pub struct CharSelectModel {
    pub char_id: i32,
    pub account_id: i32,
    pub char_num: i16,
    pub name: String,
    pub class: i16,
    pub zeny: i32,
    pub status_point: i16,
    pub str: i16,
    pub agi: i16,
    pub vit: i16,
    pub int: i16,
    pub dex: i16,
    pub luk: i16,
    pub max_hp: i32,
    pub hp: i32,
    pub max_sp: i32,
    pub sp: i32,
    pub hair: i16,
    pub hair_color: i16,
    pub last_map: String,
    pub last_x: i16,
    pub last_y: i16,
    pub save_map: String,
    pub save_x: i16,
    pub save_y: i16,
    pub sex: String,
    pub inventory_slots: i16,
    pub clothes_color: i16,
    pub body: i16,
    pub weapon: i16,
    pub shield: i16,
    pub head_top: i16,
    pub head_mid: i16,
    pub head_bottom: i16,
    pub robe: i32,
}
use sqlx::{FromRow, Error, Row};
use crate::packets::packets::CharacterInfoNeoUnion;
use sqlx::mysql::MySqlRow;

impl <'r>FromRow<'r, MySqlRow> for CharacterInfoNeoUnion {
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        let mut character_info_neo_union = CharacterInfoNeoUnion::new();
        
        character_info_neo_union.set_gid(row.get("char_id"));
        character_info_neo_union.set_exp(row.get("base_exp"));
        character_info_neo_union.set_money(row.get("zeny"));
        character_info_neo_union.set_jobexp(row.get("job_exp"));
        character_info_neo_union.set_joblevel(row.get("job_level"));
        character_info_neo_union.set_bodystate(0);
        character_info_neo_union.set_healthstate(0);
        character_info_neo_union.set_effectstate(row.get("option"));
        character_info_neo_union.set_virtue(row.get("karma"));
        character_info_neo_union.set_honor(row.get("manner"));
        character_info_neo_union.set_jobpoint(row.get("status_point"));
        character_info_neo_union.set_hp(row.get("hp"));
        character_info_neo_union.set_maxhp(row.get("max_hp"));
        character_info_neo_union.set_sp(row.get("sp"));
        character_info_neo_union.set_maxsp(row.get("max_sp"));
        character_info_neo_union.set_speed(50); // TODO make this configurable
        character_info_neo_union.set_job(row.get("class"));
        character_info_neo_union.set_head(row.get("hair"));
        character_info_neo_union.set_body(row.get("body"));
        character_info_neo_union.set_weapon(row.get("weapon"));
        character_info_neo_union.set_level(row.get("base_level"));
        character_info_neo_union.set_sppoint(row.get("skill_point"));
        character_info_neo_union.set_accessory(row.get("head_bottom"));
        character_info_neo_union.set_shield(row.get("shield"));
        character_info_neo_union.set_accessory2(row.get("head_top"));
        character_info_neo_union.set_accessory3(row.get("head_mid"));
        character_info_neo_union.set_headpalette(row.get("hair_color"));
        character_info_neo_union.set_bodypalette(row.get("clothes_color"));
        let name: String = row.get("name");
        let mut name_as_array = [0 as char; 24];
        for (i, c) in name.chars().enumerate() {
            name_as_array[i] = c;
        }
        character_info_neo_union.set_name(name_as_array);
        character_info_neo_union.set_str(row.get("str"));
        character_info_neo_union.set_agi(row.get("agi"));
        character_info_neo_union.set_vit(row.get("vit"));
        character_info_neo_union.set_int(row.get("int"));
        character_info_neo_union.set_dex(row.get("dex"));
        character_info_neo_union.set_luk(row.get("luk"));
        character_info_neo_union.set_char_num(row.get("char_num"));
        character_info_neo_union.set_haircolor(row.get("hair_color"));
        character_info_neo_union.set_b_is_changed_char_name(row.get("rename"));
        let last_map: String = row.get("last_map");
        let mut last_map_as_array = [0 as char; 16];
        for (i, c) in last_map.chars().enumerate() {
            last_map_as_array[i] = c;
        }
        character_info_neo_union.set_last_map(last_map_as_array);
        character_info_neo_union.set_delete_date(row.get("delete_date"));
        character_info_neo_union.set_robe(row.get("robe"));
        character_info_neo_union.set_slot_addon(row.get("slotchange"));
        character_info_neo_union.set_rename_addon(0);
        character_info_neo_union.set_sex(if row.get::<&str, _>("sex") == "M" { 1 } else { 0 });
        character_info_neo_union.fill_raw();
        Ok(character_info_neo_union)
    }
}
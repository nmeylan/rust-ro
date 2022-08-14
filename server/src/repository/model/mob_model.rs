use sqlx::{Error, FromRow, Row};
use sqlx::mysql::MySqlRow;

#[derive(SettersAll, Clone, Debug)]
pub struct MobModel {
    pub id: u16,
    pub name: String,
    pub level: u16,
    pub hp: u32,
    pub sp: u32,
    pub atk1: u32,
    pub atk2: u32,
    pub def: u32,
    pub mdef: u32,
    pub str: u16,
    pub agi: u16,
    pub vit: u16,
    pub int: u16,
    pub dex: u16,
    pub luk: u16,
    pub range1: u16,
    pub range2: u16,
    pub range3: u16,
    pub scale: u16,
    pub race: u16,
    pub element: u8,
    pub mode: u16,
    pub speed: u16,
    pub atk_delay: u32,
    pub atk_motion: u32,
    pub damage_motion: u32,
    pub exp: u32,
    pub jexp: u32,
    pub mvp1id: u16,
    pub mvp1per: u16,
    pub mvp2id: u16,
    pub mvp2per: u16,
    pub mvp3id: u16,
    pub mvp3per: u16,
    pub drop1id: u16,
    pub drop1per: u16,
    pub drop2id: u16,
    pub drop2per: u16,
    pub drop3id: u16,
    pub drop3per: u16,
    pub drop4id: u16,
    pub drop4per: u16,
    pub drop5id: u16,
    pub drop5per: u16,
    pub drop6id: u16,
    pub drop6per: u16,
    pub drop7id: u16,
    pub drop7per: u16,
    pub drop8id: u16,
    pub drop8per: u16,
    pub drop9id: u16,
    pub drop9per: u16,
    pub dropcardid: u16,
    pub dropcardper: u16,
}

impl Default for MobModel {
    fn default() -> Self {
        MobModel {
            id: 0,
            name: "".to_string(),
            level: 0,
            hp: 0,
            sp: 0,
            atk1: 0,
            atk2: 0,
            def: 0,
            mdef: 0,
            str: 0,
            agi: 0,
            vit: 0,
            int: 0,
            dex: 0,
            luk: 0,
            range1: 0,
            range2: 0,
            range3: 0,
            scale: 0,
            race: 0,
            element: 0,
            mode: 0,
            speed: 0,
            atk_delay: 0,
            atk_motion: 0,
            damage_motion: 0,
            exp: 0,
            jexp: 0,
            mvp1id: 0,
            mvp1per: 0,
            mvp2id: 0,
            mvp2per: 0,
            mvp3id: 0,
            mvp3per: 0,
            drop1id: 0,
            drop1per: 0,
            drop2id: 0,
            drop2per: 0,
            drop3id: 0,
            drop3per: 0,
            drop4id: 0,
            drop4per: 0,
            drop5id: 0,
            drop5per: 0,
            drop6id: 0,
            drop6per: 0,
            drop7id: 0,
            drop7per: 0,
            drop8id: 0,
            drop8per: 0,
            drop9id: 0,
            drop9per: 0,
            dropcardid: 0,
            dropcardper: 0
        }
    }
}

impl <'r>FromRow<'r, MySqlRow> for MobModel {
    fn from_row(row: &'r MySqlRow) -> Result<Self, Error> {
        let mut model = MobModel::default();
        model.set_id(row.get("id"));
        model.set_name(row.get("name_english"));
        model.set_level(row.try_get("level").unwrap_or(0));
        model.set_hp(row.try_get("hp").unwrap_or(0));
        model.set_sp(row.try_get("sp").unwrap_or(0));
        model.set_exp(row.try_get("base_exp").unwrap_or(0));
        model.set_range1(row.try_get("attack_range").unwrap_or(0));
        model.set_range2(row.try_get("skill_range").unwrap_or(0));
        model.set_range3(row.try_get("chase_range").unwrap_or(0));
        model.set_atk1(row.try_get("attack").unwrap_or(0));
        model.set_atk2(row.try_get("attack2").unwrap_or(0));
        model.set_def(row.try_get("defense").unwrap_or(0));
        model.set_mdef(row.try_get("magic_defense").unwrap_or(0));
        model.set_str(row.try_get("str").unwrap_or(0));
        model.set_agi(row.try_get("agi").unwrap_or(0));
        model.set_vit(row.try_get("vit").unwrap_or(0));
        model.set_int(row.try_get("int").unwrap_or(0));
        model.set_dex(row.try_get("dex").unwrap_or(0));
        model.set_luk(row.try_get("luk").unwrap_or(0));
        model.set_scale(row.try_get("size").unwrap_or(0));
        model.set_race(row.try_get("race").unwrap_or(0));
        model.set_element(row.try_get("element").unwrap_or(0));
        // model.set_mode(row.get("element_level")); TODO: collect all modes
        model.set_speed(row.try_get("walk_speed").unwrap_or(0));
        model.set_atk_delay(row.try_get("attack_delay").unwrap_or(0));
        model.set_atk_motion(row.try_get("attack_motion").unwrap_or(0));
        model.set_damage_motion(row.try_get("damage_motion").unwrap_or(0));
        model.set_mvp1id(row.try_get("mvpdrop1_item").unwrap_or(0));
        model.set_mvp1per(row.try_get("mvpdrop1_rate").unwrap_or(0));
        model.set_mvp2id(row.try_get("mvpdrop2_item").unwrap_or(0));
        model.set_mvp2per(row.try_get("mvpdrop2_rate").unwrap_or(0));
        model.set_mvp3id(row.try_get("mvpdrop3_item").unwrap_or(0));
        model.set_mvp3per(row.try_get("mvpdrop3_rate").unwrap_or(0));
        model.set_drop1id(row.try_get("drop1_item").unwrap_or(0));
        model.set_drop1per(row.try_get("drop1_rate").unwrap_or(0));
        model.set_drop2id(row.try_get("drop2_item").unwrap_or(0));
        model.set_drop2per(row.try_get("drop2_rate").unwrap_or(0));
        model.set_drop3id(row.try_get("drop3_item").unwrap_or(0));
        model.set_drop3per(row.try_get("drop3_rate").unwrap_or(0));
        model.set_drop4id(row.try_get("drop4_item").unwrap_or(0));
        model.set_drop4per(row.try_get("drop4_rate").unwrap_or(0));
        model.set_drop5id(row.try_get("drop5_item").unwrap_or(0));
        model.set_drop5per(row.try_get("drop5_rate").unwrap_or(0));
        model.set_drop6id(row.try_get("drop6_item").unwrap_or(0));
        model.set_drop6per(row.try_get("drop6_rate").unwrap_or(0));
        model.set_drop7id(row.try_get("drop7_item").unwrap_or(0));
        model.set_drop7per(row.try_get("drop7_rate").unwrap_or(0));
        model.set_drop8id(row.try_get("drop8_item").unwrap_or(0));
        model.set_drop8per(row.try_get("drop8_rate").unwrap_or(0));
        model.set_drop9id(row.try_get("drop9_item").unwrap_or(0));
        model.set_drop9per(row.try_get("drop9_rate").unwrap_or(0));
        model.set_dropcardid(row.try_get("drop10_item").unwrap_or(0));
        model.set_dropcardper(row.try_get("drop10_rate").unwrap_or(0));
        Ok(model)
    }
}
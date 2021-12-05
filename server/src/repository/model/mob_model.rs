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
    pub defense_motion: u32,
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
            defense_motion: 0,
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
        model.set_id(row.get("ID"));
        model.set_name(row.get("iName"));
        model.set_level(row.get("LV"));
        model.set_hp(row.get("HP"));
        model.set_sp(row.get("SP"));
        model.set_exp(row.get("EXP"));
        model.set_range1(row.get("Range1"));
        model.set_range2(row.get("Range2"));
        model.set_range3(row.get("Range3"));
        model.set_atk1(row.get("ATK1"));
        model.set_atk2(row.get("ATK2"));
        model.set_def(row.get("DEF"));
        model.set_mdef(row.get("MDEF"));
        model.set_str(row.get("STR"));
        model.set_agi(row.get("AGI"));
        model.set_vit(row.get("VIT"));
        model.set_int(row.get("INT"));
        model.set_dex(row.get("DEX"));
        model.set_luk(row.get("LUK"));
        model.set_scale(row.get("Scale"));
        model.set_race(row.get("Race"));
        model.set_element(row.get("Element"));
        model.set_mode(row.get("Mode"));
        model.set_speed(row.get("Speed"));
        model.set_atk_delay(row.get("aDelay"));
        model.set_atk_motion(row.get("aMotion"));
        model.set_defense_motion(row.get("dMotion"));
        model.set_mvp1id(row.get("MVP1id"));
        model.set_mvp1per(row.get("MVP1per"));
        model.set_mvp2id(row.get("MVP2id"));
        model.set_mvp2per(row.get("MVP2per"));
        model.set_mvp3id(row.get("MVP3id"));
        model.set_mvp3per(row.get("MVP3per"));
        model.set_drop1id(row.get("Drop1id"));
        model.set_drop1per(row.get("Drop1per"));
        model.set_drop2id(row.get("Drop2id"));
        model.set_drop2per(row.get("Drop2per"));
        model.set_drop3id(row.get("Drop3id"));
        model.set_drop3per(row.get("Drop3per"));
        model.set_drop4id(row.get("Drop4id"));
        model.set_drop4per(row.get("Drop4per"));
        model.set_drop5id(row.get("Drop5id"));
        model.set_drop5per(row.get("Drop5per"));
        model.set_drop6id(row.get("Drop6id"));
        model.set_drop6per(row.get("Drop6per"));
        model.set_drop7id(row.get("Drop7id"));
        model.set_drop7per(row.get("Drop7per"));
        model.set_drop8id(row.get("Drop8id"));
        model.set_drop8per(row.get("Drop8per"));
        model.set_drop9id(row.get("Drop9id"));
        model.set_drop9per(row.get("Drop9per"));
        model.set_dropcardid(row.get("DropCardid"));
        model.set_dropcardper(row.get("DropCardper"));
        Ok(model)
    }
}
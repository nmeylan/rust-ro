use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use crate::repository::model::item_model::ItemModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct MobModels {
    mobs: Vec<MobModel>
}
impl From<Vec<MobModel>> for MobModels {
    fn from(mobs: Vec<MobModel>) -> Self {
        MobModels {
            mobs
        }
    }
}
impl From<MobModels> for Vec<MobModel> {
    fn from(mob_models: MobModels) -> Self {
        mob_models.mobs
    }
}

#[derive(SettersAll, Clone, Debug, Serialize, Deserialize)]
pub struct MobModel {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub hp: i32,
    pub sp: i32,
    pub atk1: i32,
    pub atk2: i32,
    pub def: i32,
    pub mdef: i32,
    pub str: i32,
    pub agi: i32,
    pub vit: i32,
    pub int: i32,
    pub dex: i32,
    pub luk: i32,
    pub range1: i16,
    pub range2: i16,
    pub range3: i16,
    pub scale: i16,
    pub race: i16,
    pub element: i8,
    pub mode: i16,
    pub speed: i32,
    pub atk_delay: i32,
    pub atk_motion: i32,
    pub damage_motion: i32,
    pub exp: i32,
    pub jexp: i32,
    pub mvp1id: i16,
    pub mvp1per: i16,
    pub mvp2id: i16,
    pub mvp2per: i16,
    pub mvp3id: i16,
    pub mvp3per: i16,
    pub drop1id: i16,
    pub drop1per: i16,
    pub drop2id: i16,
    pub drop2per: i16,
    pub drop3id: i16,
    pub drop3per: i16,
    pub drop4id: i16,
    pub drop4per: i16,
    pub drop5id: i16,
    pub drop5per: i16,
    pub drop6id: i16,
    pub drop6per: i16,
    pub drop7id: i16,
    pub drop7per: i16,
    pub drop8id: i16,
    pub drop8per: i16,
    pub drop9id: i16,
    pub drop9per: i16,
    pub dropcardid: i16,
    pub dropcardper: i16,
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

impl <'r>FromRow<'r, PgRow> for MobModel {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let mut model = MobModel::default();
        model.set_id(row.get::<i32,_>("id") );
        model.set_name(row.get("name_english"));
        model.set_level(row.try_get::<i32,_>("level").unwrap_or(0));
        model.set_hp(row.try_get::<i32,_>("hp").unwrap_or(0));
        model.set_sp(row.try_get::<i32,_>("sp").unwrap_or(0));
        model.set_exp(row.try_get::<i32,_>("base_exp").unwrap_or(0));
        model.set_range1(row.try_get::<i16,_>("attack_range").unwrap_or(0));
        model.set_range2(row.try_get::<i16,_>("skill_range").unwrap_or(0));
        model.set_range3(row.try_get::<i16,_>("chase_range").unwrap_or(0));
        model.set_atk1(row.try_get::<i32,_>("attack").unwrap_or(0));
        model.set_atk2(row.try_get::<i32,_>("attack2").unwrap_or(0));
        model.set_def(row.try_get::<i32,_>("defense").unwrap_or(0));
        model.set_mdef(row.try_get::<i32,_>("magic_defense").unwrap_or(0));
        model.set_str(row.try_get::<i32,_>("str").unwrap_or(0));
        model.set_agi(row.try_get::<i32,_>("agi").unwrap_or(0));
        model.set_vit(row.try_get::<i32,_>("vit").unwrap_or(0));
        model.set_int(row.try_get::<i32,_>("int").unwrap_or(0));
        model.set_dex(row.try_get::<i32,_>("dex").unwrap_or(0));
        model.set_luk(row.try_get::<i32,_>("luk").unwrap_or(0));
        model.set_scale(row.try_get::<i16,_>("size").unwrap_or(0));
        model.set_race(row.try_get::<i16,_>("race").unwrap_or(0));
        model.set_element(row.try_get::<i8,_>("element").unwrap_or(0));
        // model.set_mode(row.get::<i32,_>("element_level")); TODO: collect all modes
        model.set_speed(row.try_get::<i32,_>("walk_speed").unwrap_or(0));
        model.set_atk_delay(row.try_get::<i32,_>("attack_delay").unwrap_or(0));
        model.set_atk_motion(row.try_get::<i32,_>("attack_motion").unwrap_or(0));
        model.set_damage_motion(row.try_get::<i32,_>("damage_motion").unwrap_or(0));
        model.set_mvp1id(row.try_get::<i16,_>("mvpdrop1_item").unwrap_or(0));
        model.set_mvp1per(row.try_get::<i16,_>("mvpdrop1_rate").unwrap_or(0));
        model.set_mvp2id(row.try_get::<i16,_>("mvpdrop2_item").unwrap_or(0));
        model.set_mvp2per(row.try_get::<i16,_>("mvpdrop2_rate").unwrap_or(0));
        model.set_mvp3id(row.try_get::<i16,_>("mvpdrop3_item").unwrap_or(0));
        model.set_mvp3per(row.try_get::<i16,_>("mvpdrop3_rate").unwrap_or(0));
        model.set_drop1id(row.try_get::<i16,_>("drop1_item").unwrap_or(0));
        model.set_drop1per(row.try_get::<i16,_>("drop1_rate").unwrap_or(0));
        model.set_drop2id(row.try_get::<i16,_>("drop2_item").unwrap_or(0));
        model.set_drop2per(row.try_get::<i16,_>("drop2_rate").unwrap_or(0));
        model.set_drop3id(row.try_get::<i16,_>("drop3_item").unwrap_or(0));
        model.set_drop3per(row.try_get::<i16,_>("drop3_rate").unwrap_or(0));
        model.set_drop4id(row.try_get::<i16,_>("drop4_item").unwrap_or(0));
        model.set_drop4per(row.try_get::<i16,_>("drop4_rate").unwrap_or(0));
        model.set_drop5id(row.try_get::<i16,_>("drop5_item").unwrap_or(0));
        model.set_drop5per(row.try_get::<i16,_>("drop5_rate").unwrap_or(0));
        model.set_drop6id(row.try_get::<i16,_>("drop6_item").unwrap_or(0));
        model.set_drop6per(row.try_get::<i16,_>("drop6_rate").unwrap_or(0));
        model.set_drop7id(row.try_get::<i16,_>("drop7_item").unwrap_or(0));
        model.set_drop7per(row.try_get::<i16,_>("drop7_rate").unwrap_or(0));
        model.set_drop8id(row.try_get::<i16,_>("drop8_item").unwrap_or(0));
        model.set_drop8per(row.try_get::<i16,_>("drop8_rate").unwrap_or(0));
        model.set_drop9id(row.try_get::<i16,_>("drop9_item").unwrap_or(0));
        model.set_drop9per(row.try_get::<i16,_>("drop9_rate").unwrap_or(0));
        model.set_dropcardid(row.try_get::<i16,_>("drop10_item").unwrap_or(0));
        model.set_dropcardper(row.try_get::<i16,_>("drop10_rate").unwrap_or(0));
        Ok(model)
    }
}
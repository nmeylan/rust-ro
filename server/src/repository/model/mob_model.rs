use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{Encode, Error, FromRow, Row};
use sqlx::postgres::PgRow;


#[derive(Debug, Serialize, Deserialize)]
pub struct MobModels {
    mobs: Vec<MobModel>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Drop {
    pub item_name: String,
    pub item_id: i32,
    pub rate: u16,
    pub is_card: bool,
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
    pub drops: Vec<Drop>,
    #[serde(default)]
    pub mvp_drops: Vec<Drop>,
}

pub fn deserialize_card<'de, D>(deserializer: D) -> Result<Option<Drop>, D::Error>
    where D: Deserializer<'de> {
    let value: Option<Drop> = Deserialize::deserialize(deserializer)?;
    Ok(value)
}

impl Default for MobModel {
    fn default() -> Self {
        let model = MobModel {
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
            drops: Default::default(),
            mvp_drops: Default::default(),
        };
        model
    }
}

impl<'r> FromRow<'r, PgRow> for MobModel {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let mut model = MobModel::default();
        model.set_id(row.get::<i32, _>("id"));
        model.set_name(row.get("name_english"));
        model.set_level(row.try_get::<i32, _>("level").unwrap_or(0));
        model.set_hp(row.try_get::<i32, _>("hp").unwrap_or(0));
        model.set_sp(row.try_get::<i32, _>("sp").unwrap_or(0));
        model.set_exp(row.try_get::<i32, _>("base_exp").unwrap_or(0));
        model.set_range1(row.try_get::<i16, _>("attack_range").unwrap_or(0));
        model.set_range2(row.try_get::<i16, _>("skill_range").unwrap_or(0));
        model.set_range3(row.try_get::<i16, _>("chase_range").unwrap_or(0));
        model.set_atk1(row.try_get::<i32, _>("attack").unwrap_or(0));
        model.set_atk2(row.try_get::<i32, _>("attack2").unwrap_or(0));
        model.set_def(row.try_get::<i32, _>("defense").unwrap_or(0));
        model.set_mdef(row.try_get::<i32, _>("magic_defense").unwrap_or(0));
        model.set_str(row.try_get::<i32, _>("str").unwrap_or(0));
        model.set_agi(row.try_get::<i32, _>("agi").unwrap_or(0));
        model.set_vit(row.try_get::<i32, _>("vit").unwrap_or(0));
        model.set_int(row.try_get::<i32, _>("int").unwrap_or(0));
        model.set_dex(row.try_get::<i32, _>("dex").unwrap_or(0));
        model.set_luk(row.try_get::<i32, _>("luk").unwrap_or(0));
        model.set_scale(row.try_get::<i16, _>("size").unwrap_or(0));
        model.set_race(row.try_get::<i16, _>("race").unwrap_or(0));
        model.set_element(row.try_get::<i8, _>("element").unwrap_or(0));
        // model.set_mode(row.get::<i32,_>("element_level")); TODO: collect all modes
        model.set_speed(row.try_get::<i32, _>("walk_speed").unwrap_or(0));
        model.set_atk_delay(row.try_get::<i32, _>("attack_delay").unwrap_or(0));
        model.set_atk_motion(row.try_get::<i32, _>("attack_motion").unwrap_or(0));
        model.set_damage_motion(row.try_get::<i32, _>("damage_motion").unwrap_or(0));
        let mut drops = vec![];
        let mut mvp_drops = vec![];
        let mut card: Option<Drop> = None;
        for i in 1..=10 {
            if let Ok(item_name) = row.try_get::<String, _>(format!("drop{}_item", i).as_str()) {
                let drop = Drop { item_name: item_name.clone(), is_card: item_name.to_lowercase().ends_with("card"), item_id: row.get::<i32, _>(format!("drop{}_itemid", i).as_str()), rate: row.get::<i32, _>(format!("drop{}_rate", i).as_str()) as u16 };
                drops.push(drop)
            } else { break; }
        }
        for i in 1..=3 {
            if let Ok(item_name) = row.try_get::<String, _>(format!("mvpdrop{}_item", i).as_str()) {
                let drop = Drop { item_name, is_card: false, item_id: row.get::<i32, _>(format!("mvpdrop{}_itemid", i).as_str()), rate: row.get::<i32, _>(format!("mvpdrop{}_rate", i).as_str()) as u16 };
                mvp_drops.push(drop)
            } else { break; }
        }
        model.set_drops(drops);
        model.set_mvp_drops(mvp_drops);
        Ok(model)
    }
}
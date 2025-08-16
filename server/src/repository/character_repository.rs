use crate::repository::model::char_model::{CharInsertModel, CharSelectModel, CharacterInfoNeoUnionWrapped};
use crate::repository::{CharacterRepository, PgRepository};
use crate::util::tick::get_tick;
use async_trait::async_trait;
use models::enums::skill_enums::SkillEnum;
use models::enums::EnumWithMaskValueU64;
use models::status::{KnownSkill, Status, StatusSnapshot};
use models::status_bonus::{StatusBonusFlag, TemporaryStatusBonuses};
use sqlx::{Error, Postgres, Row};

#[async_trait]
impl CharacterRepository for PgRepository {
    async fn character_insert(&self, char_model: &CharInsertModel) -> Result<(), Error> {
        char_model.insert(&self.pool, "char").await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn character_info(&self, account_id: i32, char_name: &str) -> Result<CharacterInfoNeoUnionWrapped, Error> {
        sqlx::query_as::<_, CharacterInfoNeoUnionWrapped>("SELECT * from char WHERE name = $1 AND account_id = $2")
            .bind(char_name)
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
    }


    async fn characters_info(&self, account_id: u32) -> Vec<CharacterInfoNeoUnionWrapped> {
        sqlx::query_as::<Postgres, CharacterInfoNeoUnionWrapped>("SELECT * FROM char WHERE account_id = $1")
            .bind(account_id as i32)
            .fetch_all(&self.pool).await.unwrap_or(vec![])
    }

    async fn character_delete_reserved(&self, account_id: u32, char_id: u32) -> Result<(), Error> {
        sqlx::query("UPDATE `char` SET delete_date = UNIX_TIMESTAMP(now() + INTERVAL 1 DAY) WHERE account_id = $1 AND char_id = $2")
            .bind(account_id as i32)
            .bind(char_id as i32)
            .execute(&self.pool).await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn character_save_position(&self, char_id: u32, map_name: String, x: u16, y: u16) -> Result<(), Error> {
        sqlx::query("UPDATE char SET last_map = $1, last_x = $2, last_y = $3 WHERE char_id = $4")
            .bind(map_name)
            .bind(x as i16)
            .bind(y as i16)
            .bind(char_id as i32)
            .execute(&self.pool)
            .await
            .inspect_err(|e| {
                error!("DB error: {:?}", e);
            })
            .map(|_| ())
    }

    async fn character_update_status(&self, char_id: u32, db_column: String, value: u32) -> Result<(), Error> {
        let sql = format!("UPDATE char SET {db_column} = $1 WHERE char_id = $2"); // TODO sanitize db_column
        sqlx::query(&sql).bind(value as i32).bind(char_id as i32).execute(&self.pool).await
            .inspect_err(|e| {
                error!("DB error: {:?}", e);
            })
            .map(|_| ())
    }

    async fn character_zeny_fetch(&self, char_id: u32) -> Result<i32, Error> {
        sqlx::query("SELECT zeny FROM char WHERE char_id = $1")
            .bind(char_id as i32)
            .fetch_one(&self.pool).await.map(|row| Ok(row.get::<i32, _>(0)))?
    }

    async fn character_allocated_skill_points(&self, char_id: u32) -> Result<i32, Error> {
        sqlx::query("SELECT sum(lv) FROM skill WHERE char_id = $1")
            .bind(char_id as i32)
            .fetch_one(&self.pool).await.map(|row| Ok(row.get::<i32, _>(0)))?
    }

    async fn character_skills(&self, char_id: u32) -> Result<Vec<KnownSkill>, Error> {
        sqlx::query("SELECT id, lv FROM skill WHERE char_id = $1")
            .bind(char_id as i32)
            .fetch_all(&self.pool).await
            .map(|rows| rows.iter().map(|row| KnownSkill { value: SkillEnum::from_id(row.get::<i32, _>(0) as u32), level: row.get::<i16, _>(1) as u8 }).collect::<Vec<KnownSkill>>())
    }

    async fn character_fetch(&self, account_id: u32, char_num: u8) -> Result<CharSelectModel, Error> {
        sqlx::query_as::<_, CharSelectModel>("SELECT * FROM char WHERE account_id = $1 AND char_num = $2")
            .bind(account_id as i32)
            .bind(char_num as i16)
            .fetch_one(&self.pool).await
    }

    async fn character_with_id_fetch(&self, char_id: u32) -> Result<CharSelectModel, Error> {
        sqlx::query_as::<_, CharSelectModel>("SELECT * FROM char WHERE char_id = $1")
            .bind(char_id as i32)
            .fetch_one(&self.pool).await
    }


    async fn character_reset_skills(&self, char_id: i32, skills: Vec<i32>) -> Result<(), Error> {
        sqlx::query("DELETE FROM skill WHERE char_id = $1 and id IN (SELECT * FROM UNNEST($2::int4[]))").bind(char_id).bind(skills).execute(&self.pool).await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn character_allocate_skill_point(&self, char_id: i32, skill_id: i32, increment: u8) -> Result<(), Error> {
        sqlx::query("INSERT INTO skill (char_id, id, lv, flag) values ($1, $2, $3, 0) ON CONFLICT (char_id, id) DO UPDATE set lv = skill.lv + EXCLUDED.lv")
            .bind(char_id).bind(skill_id).bind(increment as i16).execute(&self.pool).await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn characters_update(&self, statuses: Vec<&Status>, statuses_snaphot: Vec<StatusSnapshot>, char_ids: Vec<i32>, x: Vec<i16>, y: Vec<i16>, maps: Vec<String>) -> Result<(), Error> {
        let classes: Vec<i16> = statuses_snaphot.iter().map(|s| s.job() as i16).collect();
        let base_levels: Vec<i32> = statuses.iter().map(|s| s.base_level() as i32).collect();
        let job_levels: Vec<i32> = statuses.iter().map(|s| s.job_level() as i32).collect();
        let base_exps: Vec<i32> = statuses.iter().map(|s| s.job_exp() as i32).collect();
        let job_exps: Vec<i32> = statuses.iter().map(|s| s.job_exp() as i32).collect();
        let zenys: Vec<i32> = statuses.iter().map(|s| s.zeny() as i32).collect();
        let strs: Vec<i16> = statuses_snaphot.iter().map(|s| s.base_str() as i16).collect();
        let agis: Vec<i16> = statuses_snaphot.iter().map(|s| s.base_agi() as i16).collect();
        let vits: Vec<i16> = statuses_snaphot.iter().map(|s| s.base_vit() as i16).collect();
        let ints: Vec<i16> = statuses_snaphot.iter().map(|s| s.base_int() as i16).collect();
        let dexs: Vec<i16> = statuses_snaphot.iter().map(|s| s.base_dex() as i16).collect();
        let luks: Vec<i16> = statuses_snaphot.iter().map(|s| s.base_luk() as i16).collect();
        let max_hps: Vec<i32> = statuses_snaphot.iter().map(|s| s.max_hp() as i32).collect();
        let hps: Vec<i32> = statuses_snaphot.iter().map(|s| s.hp() as i32).collect();
        let max_sps: Vec<i32> = statuses_snaphot.iter().map(|s| s.max_sp() as i32).collect();
        let sps: Vec<i32> = statuses_snaphot.iter().map(|s| s.sp() as i32).collect();
        let status_points: Vec<i16> = statuses.iter().map(|s| s.status_point() as i16).collect();
        let skill_points: Vec<i16> = statuses.iter().map(|s| s.skill_point() as i16).collect();
        let hairs: Vec<i16> = statuses.iter().map(|s| s.look().map(|l| l.hair() as i16).unwrap_or(0_i16)).collect();
        let hair_colors: Vec<i16> = statuses.iter().map(|s| s.look().map(|l| l.hair_color() as i16).unwrap_or(0_i16)).collect();
        let clothes_colors: Vec<i16> = statuses.iter().map(|s| s.look().map(|l| l.clothes_color() as i16).unwrap_or(0_i16)).collect();
        let bodies: Vec<i16> = statuses.iter().map(|s| s.armor().map(|e| e.item_id as i16).unwrap_or(0_i16)).collect();
        let weapons: Vec<i16> = statuses.iter().map(|s| s.right_hand_weapon().map(|w| w.item_id() as i16).unwrap_or(0_i16)).collect();
        let shields: Vec<i16> = statuses.iter().map(|s| s.shield().map(|e| e.item_id() as i16).unwrap_or(0_i16)).collect();
        let head_tops: Vec<i16> = statuses.iter().map(|s| s.head_top().map(|e| e.item_id as i16).unwrap_or(0_i16)).collect();
        let head_mids: Vec<i16> = statuses.iter().map(|s| s.head_mid().map(|e| e.item_id as i16).unwrap_or(0_i16)).collect();
        let head_bottoms: Vec<i16> = statuses.iter().map(|s| s.head_low().map(|e| e.item_id as i16).unwrap_or(0_i16)).collect();
        let robes: Vec<i32> = statuses.iter().map(|s| s.look().map(|l| l.robe() as i32).unwrap_or(0_i32)).collect();
        
        let last_maps: Vec<String> = maps;
        let last_xs: Vec<i16> = x;
        let last_ys: Vec<i16> = y;

        // Use UNNEST to perform the bulk update
        let query = r#"
        UPDATE ragnarok."char" AS c
        SET
            class = u.class,
            base_level = u.base_level,
            job_level = u.job_level,
            base_exp = u.base_exp,
            job_exp = u.job_exp,
            zeny = u.zeny,
            str = u.str,
            agi = u.agi,
            vit = u.vit,
            "int" = u.int,
            dex = u.dex,
            luk = u.luk,
            max_hp = u.max_hp,
            hp = u.hp,
            max_sp = u.max_sp,
            sp = u.sp,
            status_point = u.status_point,
            skill_point = u.skill_point,
            hair = u.hair,
            hair_color = u.hair_color,
            clothes_color = u.clothes_color,
            body = u.body,
            weapon = u.weapon,
            shield = u.shield,
            head_top = u.head_top,
            head_mid = u.head_mid,
            head_bottom = u.head_bottom,
            robe = u.robe,
            last_map = u.last_map,
            last_x = u.last_x,
            last_y = u.last_y
        FROM (
            SELECT
                UNNEST($1::SMALLINT[]) AS class,
                UNNEST($2::INTEGER[]) AS base_level,
                UNNEST($3::INTEGER[]) AS job_level,
                UNNEST($4::INTEGER[]) AS base_exp,
                UNNEST($5::INTEGER[]) AS job_exp,
                UNNEST($6::INTEGER[]) AS zeny,
                UNNEST($7::SMALLINT[]) AS str,
                UNNEST($8::SMALLINT[]) AS agi,
                UNNEST($9::SMALLINT[]) AS vit,
                UNNEST($10::SMALLINT[]) AS int,
                UNNEST($11::SMALLINT[]) AS dex,
                UNNEST($12::SMALLINT[]) AS luk,
                UNNEST($13::INTEGER[]) AS max_hp,
                UNNEST($14::INTEGER[]) AS hp,
                UNNEST($15::INTEGER[]) AS max_sp,
                UNNEST($16::INTEGER[]) AS sp,
                UNNEST($17::SMALLINT[]) AS status_point,
                UNNEST($18::SMALLINT[]) AS skill_point,
                UNNEST($19::SMALLINT[]) AS hair,
                UNNEST($20::SMALLINT[]) AS hair_color,
                UNNEST($21::SMALLINT[]) AS clothes_color,
                UNNEST($22::SMALLINT[]) AS body,
                UNNEST($23::SMALLINT[]) AS weapon,
                UNNEST($24::SMALLINT[]) AS shield,
                UNNEST($25::SMALLINT[]) AS head_top,
                UNNEST($26::SMALLINT[]) AS head_mid,
                UNNEST($27::SMALLINT[]) AS head_bottom,
                UNNEST($28::INTEGER[]) AS robe,
                UNNEST($29::VARCHAR[]) AS last_map,
                UNNEST($30::SMALLINT[]) AS last_x,
                UNNEST($31::SMALLINT[]) AS last_y,
                UNNEST($32::INTEGER[]) AS char_id
        ) AS u
        WHERE c.char_id = u.char_id
    "#;
        sqlx::query(query)
            .bind(&classes)
            .bind(&base_levels)
            .bind(&job_levels)
            .bind(&base_exps)
            .bind(&job_exps)
            .bind(&zenys)
            .bind(&strs)
            .bind(&agis)
            .bind(&vits)
            .bind(&ints)
            .bind(&dexs)
            .bind(&luks)
            .bind(&max_hps)
            .bind(&hps)
            .bind(&max_sps)
            .bind(&sps)
            .bind(&status_points)
            .bind(&skill_points)
            .bind(&hairs)
            .bind(&hair_colors)
            .bind(&clothes_colors)
            .bind(&bodies)
            .bind(&weapons)
            .bind(&shields)
            .bind(&head_tops)
            .bind(&head_mids)
            .bind(&head_bottoms)
            .bind(&robes)
            .bind(&last_maps)
            .bind(&last_xs)
            .bind(&last_ys)
            .bind(&char_ids)
            .execute(&self.pool)
            .await
            .inspect_err(|e| {
                error!("DB error: {:?}", e);
            })
            .map(|_| ())
    }

    async fn characters_list_for_simulator(&self) -> Result<Vec<CharSelectModel>, Error> {
        sqlx::query_as::<_, CharSelectModel>("SELECT * from char limit 100")
            .fetch_all(&self.pool)
            .await
    }

    async fn character_save_temporary_bonus(&self, char_id: u32, account_id: u32, temporary_bonuses: &TemporaryStatusBonuses) -> Result<(), Error> {
        sqlx::query("DELETE FROM ragnarok.sc_data WHERE char_id = $1")
            .bind(char_id as i32)
            .execute(&self.pool)
            .await
            .inspect_err(|e| {
                error!("DB error: {:?}", e);
            })?;

        let persist_bonuses: Vec<_> = temporary_bonuses.iter()
            .filter(|bonus| bonus.flags() & StatusBonusFlag::Persist.as_flag() > 0)
            .collect();

        if persist_bonuses.is_empty() {
            return Ok(());
        }

        let mut account_ids: Vec<i32> = Vec::new();
        let mut char_ids: Vec<i32> = Vec::new();
        let mut types: Vec<i32> = Vec::new();
        let mut ticks: Vec<i32> = Vec::new();
        let mut val1s: Vec<i32> = Vec::new();
        let mut val2s: Vec<i32> = Vec::new();
        let mut val3s: Vec<i32> = Vec::new();
        let mut val4s: Vec<i32> = Vec::new();

        let now = get_tick();
        for bonus in persist_bonuses {
            let (bonus_type, val1, val2, val3, val4) = bonus.bonus().serialize_to_sc_data();
            let tick = match bonus.expirency() {
                models::status_bonus::BonusExpiry::Time(until) => if *until > now { *until - now  } else { 0 },
                models::status_bonus::BonusExpiry::Never => 0,
                models::status_bonus::BonusExpiry::Counter(_) => 0,
            };
            if tick == 0 {
                continue;
            }

            account_ids.push(account_id as i32);
            char_ids.push(char_id as i32);
            types.push(bonus_type);
            ticks.push(tick as i32);
            val1s.push(val1);
            val2s.push(val2);
            val3s.push(val3);
            val4s.push(val4);
        }

        let query = r#"
        INSERT INTO ragnarok.sc_data (account_id, char_id, type, tick, val1, val2, val3, val4)
        SELECT * FROM UNNEST($1::INTEGER[], $2::INTEGER[], $3::INTEGER[], $4::INTEGER[], $5::INTEGER[], $6::INTEGER[], $7::INTEGER[], $8::INTEGER[])
        "#;

        sqlx::query(query)
            .bind(&account_ids)
            .bind(&char_ids)
            .bind(&types)
            .bind(&ticks)
            .bind(&val1s)
            .bind(&val2s)
            .bind(&val3s)
            .bind(&val4s)
            .execute(&self.pool)
            .await
            .inspect_err(|e| {
                error!("DB error: {:?}", e);
            })
            .map(|_| ())
    }
}
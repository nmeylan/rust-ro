use accessor::SettersAll;


#[derive(SettersAll, Debug, Clone, Copy)]
pub struct Status {
    pub job: u32,
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    pub str: u16,
    pub agi: u16,
    pub vit: u16,
    pub int: u16,
    pub dex: u16,
    pub luk: u16,
    pub base_atk: u32,
    pub matk_min: u32,
    pub matk_max: u32,
    pub speed: u16,
    pub hit: u32,
    pub flee: u32,
    pub crit: u32,
    pub def: u32,
    pub mdef: u32,
    pub look: Option<Look>,
    pub zeny: u32,
    pub base_level: u32,
    pub job_level: u32,
    pub status_point: u32,
    pub skill_point: u32,
    pub base_exp: u32,
    pub job_exp: u32,
}

#[derive(SettersAll, Debug, Clone, Copy, Default)]
pub struct Look {
    pub hair: u16,
    pub hair_color: u32,
    pub clothes_color: u32,
    pub body: u32,
    pub weapon: u32,
    pub shield: u32,
    pub head_top: u32,
    pub head_middle: u32,
    pub head_bottom: u32,
    pub robe: u32,
}

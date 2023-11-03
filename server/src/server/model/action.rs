use skills::Skill;

#[derive(Clone, Copy, Debug)]
pub struct Attack {
    pub target: u32,
    pub repeat: bool,
    pub last_attack_tick: u128,
    pub last_attack_motion: u32
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Damage {
    pub target_id: u32,
    pub attacker_id: u32,
    pub damage: u32,
    pub attacked_at: u128,
}

pub struct SkillInUse {
    pub target: Option<u32>,
    pub start_skill_tick: u128,
    pub skill: Box<dyn Skill>,
    pub used_at_tick: Option<u128>, // when the skill was actually used
}
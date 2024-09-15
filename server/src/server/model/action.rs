use models::enums::skill::SkillType;
use models::enums::status::StatusEffect;
use models::status_bonus::TemporaryStatusBonuses;
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

#[derive(Debug, PartialEq, Clone)]
pub struct AddBonuses {
    pub target_id: u32,
    pub effects: Vec<StatusEffect>,
    pub bonuses: TemporaryStatusBonuses,
}

pub struct SkillInUse {
    pub target: Option<u32>,
    pub start_skill_tick: u128,
    pub skill: Box<dyn Skill>,
    pub used_at_tick: Option<u128>, // when the skill was actually used
}

pub struct SkillUsed {
    pub skill_type: SkillType,
    pub source_id: u32,
    pub target_id: u32,
    // When negative it heals target
    pub damage_to_target: i32,
    // When negative it heals self
    pub damage_to_self: i32,
    pub effects: Vec<StatusEffect>,
    pub bonuses: TemporaryStatusBonuses,
    // For offensive skills
    pub attacked_at: u128,
}

impl SkillUsed {
    pub fn to_damage(&self) -> Damage {
        Damage {
            target_id: self.target_id,
            attacker_id: self.source_id,
            damage: self.damage_to_target as u32,
            attacked_at: self.attacked_at,
        }
    }
}
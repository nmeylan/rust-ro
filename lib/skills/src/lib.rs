use enums::weapon::WeaponType;

pub mod skills;


pub trait Skill {
    fn validate_sp(&self, level: u8, character_sp: u32);
    fn validate_hp(&self, level: u8, character_hp: u32);
    fn validate_level(&self, level: u8, character_level: u32);
    fn validate_weapon(&self, level: u8, character_weapon: WeaponType);

}

pub trait OffensiveSkill : Skill {
    fn hit_count(&self, level: u8);
    fn cast_time(&self, level: u8);

    fn damage(&self, level: u8);
}

pub trait SupportiveSkill {

}

struct A;

impl Skill for A {
    fn validate_sp(&self, level: u8, character_sp: u32) {
        todo!()
    }

    fn validate_hp(&self, level: u8, character_hp: u32) {
        todo!()
    }

    fn validate_level(&self, level: u8, character_level: u32) {
        todo!()
    }

    fn validate_weapon(&self, level: u8, character_weapon: WeaponType) {
        todo!()
    }
}

impl OffensiveSkill for A {
    fn hit_count(&self, level: u8) {
        todo!()
    }

    fn cast_time(&self, level: u8) {
        todo!()
    }

    fn damage(&self, level: u8) {
        todo!()
    }
}
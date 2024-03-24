
pub mod item;
pub mod position;
pub mod status;
pub mod enums;

#[cfg(test)]
mod tests {
    use crate::enums::EnumStackable;
    use enum_macro::{WithStackable, WithEq};

    #[derive(PartialEq, Clone, Copy)]
    enum EElement {
        Fire,
        Water
    }

    #[derive(WithEq, WithStackable)]
    enum AB {
        EnableSkill,
        Str(i8),
        Atk(i8),
        SkillIdIncrease(u32, i32),
        #[value_comparison_offset = 1]
        #[value_offset = 0]
        ElementPercentage(u16, EElement),
        WeaponElement(EElement),
    }
    #[test]
    fn a() {
        let ab = AB::EnableSkill;
    }
}
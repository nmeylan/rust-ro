
pub mod item;
pub mod position;
pub mod status;
pub mod enums;


#[cfg(test)]
mod tests {
    use enum_macro::WithEq;

    #[derive(PartialEq)]
    enum EElement {
        Fire,
        Water
    }

    #[derive(WithEq)]
    enum AB {
        EnableSkill,
        Str(i8),
        Atk(i8),
        SkillIdIncrease(u32, i8),
        #[value_comparison_offset = 1]
        ElementPercentage(i8, EElement),
    }
}
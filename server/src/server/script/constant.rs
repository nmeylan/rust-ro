use rathena_script_lang_interpreter::lang::value::Value;

use crate::server::core::status::LookType;

/*
([A-Z_]*): (.*)
"$1" => Value::new_number($2 as i32),
 */
pub fn load_constant(constant_name: &String) -> Option<Value> {
    let constant_value = match constant_name.as_ref() {
        // LOOK
        "LOOK_HAIR" => Value::new_number(LookType::Hair.value() as i32),
        "LOOK_WEAPON" => Value::new_number(LookType::Weapon.value() as i32),
        "LOOK_HEAD_BOTTOM" => Value::new_number(LookType::HeadBottom.value() as i32),
        "LOOK_HEAD_TOP" => Value::new_number(LookType::HeadTop.value() as i32),
        "LOOK_HEAD_MID" => Value::new_number(LookType::HeadMid.value() as i32),
        "LOOK_HAIR_COLOR" => Value::new_number(LookType::HairColor.value() as i32),
        "LOOK_CLOTHES_COLOR" => Value::new_number(LookType::ClothesColor.value() as i32),
        "LOOK_SHIELD" => Value::new_number(LookType::Shield.value() as i32),
        "LOOK_ROBE" => Value::new_number(LookType::Robe.value() as i32),
        "LOOK_BODY2" => Value::new_number(LookType::Body.value() as i32),
        // Colors
        "C_AQUA" => Value::new_number(0x00FFFF as i32),
        "C_BLACK" => Value::new_number(0x000000 as i32),
        "C_BLUE" => Value::new_number(0x0000FF as i32),
        "C_GRAY" => Value::new_number(0x808080 as i32),
        "C_GREEN" => Value::new_number(0x008000 as i32),
        "C_LIME" => Value::new_number(0x00FF00 as i32),
        "C_MAROON" => Value::new_number(0x800000 as i32),
        "C_NAVY" => Value::new_number(0x000080 as i32),
        "C_OLIVE" => Value::new_number(0x808000 as i32),
        "C_ORANGE" => Value::new_number(0xFFA500 as i32),
        "C_PURPLE" => Value::new_number(0x800080 as i32),
        "C_RED" => Value::new_number(0xFF0000 as i32),
        "C_SILVER" => Value::new_number(0xC0C0C0 as i32),
        "C_SPRINGGREEN" => Value::new_number(0x00FF99 as i32),
        "C_TEAL" => Value::new_number(0x008080 as i32),
        "C_WHITE" => Value::new_number(0xFFFFFF as i32),
        "C_YELLOW" => Value::new_number(0xFFFF00 as i32),
        "C_PINK" => Value::new_number(0xFFC0CB as i32),
        "C_CHOCOLATE" => Value::new_number(0xD2691E as i32),
        "C_GOLD" => Value::new_number(0xFFD700 as i32),
        "C_VIOLET" => Value::new_number(0xEE82EE as i32),
        // strcharinfo
        "PC_NAME" => Value::new_number(0 as i32),
        "PC_PARTY" => Value::new_number(1 as i32),
        "PC_GUILD" => Value::new_number(2 as i32),
        "PC_MAP" => Value::new_number(3 as i32),
        "PC_CLAN" => Value::new_number(4 as i32),
        &_ => Value::Reference(None)
    };
    if constant_value.is_reference() {
        None
    } else {
        Some(constant_value)
    }
}

pub fn get_battle_flag(flag_name: &String) -> Value {
    match flag_name.as_ref() {
        "min_hair_style" => Value::new_number(0),
        "max_hair_style" => Value::new_number(29),
        "min_hair_color" => Value::new_number(0),
        "max_hair_color" => Value::new_number(8),
        "min_cloth_color" => Value::new_number(0),
        "max_cloth_color" => Value::new_number(4),
        &_ => panic!("unknown battle flag {}", flag_name)
    }
}
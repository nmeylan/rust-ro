use rathena_script_lang_interpreter::lang::value::Value;

use crate::server::core::status::LookType;

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
use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use lazy_static::lazy_static;
use regex::{Regex, Captures};

use crate::{PacketStructDefinition, StructDefinition, StructField, Type};
use std::path::Path;

lazy_static! {
    pub static ref   TYPES_MAP: HashMap<&'static str, Type> = HashMap::from([
        ("char", Type {name: "i8".to_string(), cname: "char".to_string(), length: Some(1)}),
        ("unsigned char", Type {name: "u8".to_string(), cname: "unsigned char".to_string(), length: Some(1)}),
        ("unsigned byte", Type {name: "u8".to_string(), cname: "unsigned char".to_string(), length: Some(1)}),
        ("rust char", Type {name: "char".to_string(), cname: "char".to_string(), length: Some(1)}),
        ("short", Type {name: "i16".to_string(), cname: "short".to_string(), length: Some(2)}),
        ("unsigned short", Type {name: "u16".to_string(), cname: "unsigned short".to_string(), length: Some(2)}),
        ("int", Type {name: "i32".to_string(), cname: "int".to_string(), length: Some(4)}),
        ("unsigned int", Type {name: "u32".to_string(), cname: "unsigned int".to_string(), length: Some(4)}),
        ("long", Type {name: "i32".to_string(), cname: "long".to_string(), length: Some(4)}),
        ("unsigned long", Type {name: "u32".to_string(), cname: "unsigned long".to_string(), length: Some(4)}),
        ("int64", Type {name: "i64".to_string(), cname: "int64".to_string(), length: Some(8)}),
        ("unsigned int64", Type {name: "u64".to_string(), cname: "unsigned int64".to_string(), length: Some(8)}),
        ("bool", Type {name: "bool".to_string(), cname: "bool".to_string(), length: Some(1)}),
        ("string", Type {name: "String".to_string(), cname: "char[]".to_string(), length: None}),
        ("struct", Type {name: "Struct".to_string(), cname: "struct".to_string(), length: None}),
        ("array of struct", Type {name: "Vec".to_string(), cname: "[]".to_string(), length: None}),
        ("array", Type {name: "Array".to_string(), cname: "[]".to_string(), length: None}),
    ]);
    static ref STRUCT_REGEX: Regex = Regex::new(r"struct\s([^\s]*)\s.*").unwrap();
    static ref NESTED_STRUCT_REGEX: Regex = Regex::new(r"struct\s([^\s]*)\s([^\s\[]*)\[?.*/?\s(\d+)?").unwrap();
    static ref STRING_LEN_REGEX: Regex = Regex::new(r"\w*\[(\d*)\]").unwrap();
    static ref AFTER_UNDERSCORE_CHAR_REGEX: Regex = Regex::new(r"_(\w)").unwrap();
    static ref UPPERCASE_CHAR_REGEX: Regex = Regex::new(r"([A-Z])").unwrap();
    static ref FIRST_CHAR_REGEX: Regex = Regex::new(r"^(\w)").unwrap();
    static ref ARRAY_REGEX: Regex = Regex::new(r"\s([A-Za-z_0-9]*)\[(\d+)\]").unwrap();
    static ref ARRAY_OF_UNKNOWN_LENGTH_REGEX: Regex = Regex::new(r"\s([A-Za-z_0-9]*)\[...\]").unwrap();
}

pub fn parse(packet_db_path: &Path) -> (Vec<PacketStructDefinition>, Vec<StructDefinition>) {
    let file = File::open(packet_db_path).unwrap();
    let reader = io::BufReader::new(file);
    let mut packets: Vec<PacketStructDefinition> = Vec::new();
    let mut nested_structures: Vec<StructDefinition> = Vec::new();
    let mut id: String = "null".to_string();
    let mut structs_for_packet: Vec<RefCell<StructDefinition>> = Vec::new(); // packets_db can contain nested structures
    let mut current_structure_def = 0;
    for line in reader.lines() {
        let line_content = line.unwrap().trim().to_string();
        if line_content.starts_with("0x") { // new packet definition
            id = line_content.clone();
            current_structure_def = 0;
            structs_for_packet = Vec::new();
        } else if line_content.contains("struct") && structs_for_packet.len() > 0 { // start of nested struct
            let name = STRUCT_REGEX.captures(line_content.as_str()).unwrap().get(1).unwrap();
            let current_packet = structs_for_packet.get_mut(current_structure_def).unwrap().get_mut();
            current_packet.fields.push( // register this nested struct a field of current struct
                              get_field_for_nested_struct(line_content.clone(), current_packet.current_field_position.clone()));
            structs_for_packet.push(RefCell::new(StructDefinition {
                name: name.as_str().to_string(),
                current_field_position: 0,
                fields: Vec::new()
            }));
            current_structure_def += 1;
        } else if line_content.contains("struct") { // start of "main" struct. matching: struct PACKET_HC_ACCEPT_MAKECHAR_NEO_UNION {
            let name = STRUCT_REGEX.captures(line_content.as_str()).unwrap().get(1).unwrap();
            structs_for_packet.push(RefCell::new(StructDefinition {
                name: name.as_str().to_string(),
                current_field_position: 0,
                fields: Vec::new()
            }));
        } else if line_content.contains("}") { // end of a struct
            let struct_def_ref = structs_for_packet.get(current_structure_def).unwrap().borrow_mut();
            if current_structure_def > 0 { // meaning we are in a nested field
                let struct_def_copy = copy_struct_definition(struct_def_ref);
                if nested_structures.iter().find(|st| st.name.clone().as_str() == struct_def_copy.name.as_str()).is_none() &&
                    packets.iter().find(|st| st.struct_def.name.clone().as_str() == struct_def_copy.name.as_str()).is_none(){ // if struct not already present as it might be defined multiple times in packets_db
                    nested_structures.push(struct_def_copy);
                }
                current_structure_def -= 1;
            } else {
                packets.push(PacketStructDefinition {
                    id: id.clone(),
                    struct_def: copy_struct_definition(struct_def_ref),
                })
            }
        } else if !line_content.contains("struct") && !line_content.is_empty() { // any fields. matching: unsigned long GID
            // line are as following: unsigned long GID
            let current_packet = structs_for_packet.get_mut(current_structure_def).unwrap().get_mut();
            let packet_field = get_field(line_content, current_packet.current_field_position.clone());
            current_packet.increment_current_field_position(packet_field.length.clone());
            current_packet.fields.push(packet_field);
        }
    }
    (packets, nested_structures)
}

fn copy_struct_definition<'a>(struct_def_ref: RefMut<StructDefinition<'a>>) -> StructDefinition<'a> {
    StructDefinition {
        name: struct_name(&struct_def_ref.name.clone()),
        current_field_position: struct_def_ref.current_field_position.clone(),
        fields: struct_def_ref.fields.clone()
    }
}

fn struct_name(name: &String) -> String {
    if !name.contains("_") {
        return name.clone();
    }
    let mut new_name = name.to_lowercase();
    new_name = AFTER_UNDERSCORE_CHAR_REGEX.replace_all(&new_name, |caps: &Captures| { caps.get(1).unwrap().as_str().to_uppercase() }).to_string();
    new_name = FIRST_CHAR_REGEX.replace_all(&new_name, |caps: &Captures| { caps.get(1).unwrap().as_str().to_uppercase() }).to_string();
    new_name
}

fn get_field_for_nested_struct<'a>(line: String, position: i16) -> StructField<'a> {
    let nested_struct_matches = NESTED_STRUCT_REGEX.captures(line.as_str()).unwrap();
    let complex_type_name = nested_struct_matches.get(1).unwrap().as_str().to_string();
    let name = nested_struct_matches.get(2).unwrap().as_str().to_string();
    let mut length: i16 = -1;
    let length_match = nested_struct_matches.get(3);
    if length_match.is_some() {
        length = length_match.unwrap().as_str().parse::<i16>().unwrap();
    }
    StructField {
        name: get_field_name(&name),
        position,
        data_type: if line.contains("[") {   TYPES_MAP.get("array of struct").unwrap() } else {   TYPES_MAP.get("struct").unwrap() },
        length,
        complex_type: Some(struct_name(&complex_type_name)),
        sub_type: None,
    }
}

fn get_field<'a>(field_line: String, position: i16) -> StructField<'a> {
    let mut field_type = get_type(&field_line, false);
    let name = get_field_name(&field_line);
    let mut length: i16 = -1;
    let mut sub_type = None;
    if field_type.length.is_some() {
        length = field_type.length.unwrap();
    } else if field_type.name == "String" {
        length = get_string_field_length(&field_line);
    }

    if field_type.name ==  "Array" {
        let captures_option = ARRAY_REGEX.captures(field_line.as_str());
        if captures_option.is_some() { // match xxx[12]
            let options = captures_option.unwrap();
            if field_line.contains("char") {
                sub_type = Some(  TYPES_MAP.get("rust char").unwrap());
            } else {
                sub_type = Some(get_type(&field_line, true));
            }
            length = options.get(2).unwrap().as_str().parse::<i16>().unwrap();
        } else if field_line.contains("char") { // match char xxx[...]
            field_type =   TYPES_MAP.get("string").unwrap();
        }
    }

    StructField {
        name,
        position,
        data_type: field_type,
        length,
        complex_type: None,
        sub_type
    }
}

fn get_string_field_length(line: &String) -> i16 {
    let frag: Vec<&str> = line.split(" ").collect();
    let name = frag[frag.len() - 1].to_string();
    let string_len = STRING_LEN_REGEX.captures(name.as_str());
    if string_len.is_some() {
        string_len.unwrap().get(1).unwrap().as_str().parse::<i16>().unwrap()
    } else {
        -1
    }
}

fn get_field_name(line: &String) -> String {
    let frag: Vec<&str> = line.split(" ").collect();
    let mut name = frag[frag.len() - 1].to_string();
    if name.find("[").is_some() {
        name = name.chars().take(name.find("[").unwrap()).collect();
    }
    let mut index = 0;
    let mut previous_was_uppercase = false;
    // remove first letter when in capital
    // down case abbreviation: ID -> id; clientID -> clientId; AID -> aid
    name = name.chars().map(|c| {
        let mut cleaned_char = c;
        let current_is_uppercase = c.is_uppercase();
        if index == 0 || previous_was_uppercase {
            cleaned_char = c.to_ascii_lowercase()
        }
        previous_was_uppercase = current_is_uppercase;
        index = index + 1;
        cleaned_char
    }).collect();
    if name == "type" {
        name = "atype".to_string();
    }
    name = FIRST_CHAR_REGEX.replace_all(&name, |caps: &Captures| { caps.get(1).unwrap().as_str().to_ascii_lowercase() }).to_string();
    name = UPPERCASE_CHAR_REGEX.replace_all(&name, |caps: &Captures| { "_".to_owned() + caps.get(1).unwrap().as_str().to_ascii_lowercase().as_str() }).to_string();
    name
}

fn get_type(line: &String, should_ignore_array: bool) -> &'static Type {
    let is_unsigned = line.contains("unsigned");
    let type_str = line.replace("unsigned ", "");
    if type_str.contains("[") && !should_ignore_array {
        return   TYPES_MAP.get("array").unwrap();
    }
    let frag: Vec<&str> = type_str.split(" ").collect();
    let mut type_to_retrieve = frag[0].trim().to_string();
    if is_unsigned {
        type_to_retrieve = format!("unsigned {}", type_to_retrieve);
    }
    let found_type =   TYPES_MAP.get(type_to_retrieve.as_str());
    if found_type.is_none() {
        panic!("type {} not found in   TYPES_MAP", type_to_retrieve);
    }
    return found_type.unwrap();
}
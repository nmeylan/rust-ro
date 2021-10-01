use crate::{PacketStructDefinition, StructDefinition, StructField};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::packet_db_parser::static_types_map;
use byteorder::WriteBytesExt;

pub fn write_packets_struct(packets: Vec<PacketStructDefinition>, nested_structures: &Vec<StructDefinition>, output_path: &Path) {
    let mut file_packets_res = File::create(output_path.join("packets.rs"));
    let mut file_packets_impl_res = File::create(output_path.join("packets_impl.rs"));
    let mut file_print_res = File::create(output_path.join("packets_print.rs"));
    let mut file_packets_parser_res = File::create(output_path.join("packets_parser.rs"));
    let mut file_packets = file_packets_res.unwrap();
    let mut file_packets_print = file_print_res.unwrap();
    let mut file_packets_impl = file_packets_impl_res.unwrap();
    let mut file_packets_parser = file_packets_parser_res.unwrap();
    write_file_header(&mut file_packets);
    write_file_header(&mut file_packets_print);
    write_file_header(&mut file_packets_impl);
    write_file_header(&mut file_packets_parser);

    file_packets.write(b"use std::any::Any;\n\n");

    file_packets_print.write(b"use crate::packets::packets::*;\n");
    file_packets_print.write(b"use std::fmt::{Formatter, Debug, Display};\n");
    file_packets_print.write(b"use crate::util::print::PrettyOutput;\n\n");

    file_packets_parser.write(b"use crate::packets::packets::*;\n\n");

    file_packets_impl.write(b"#![allow(dead_code)]\n\n");
    file_packets_impl.write(b"use crate::packets::packets::*;\n");
    file_packets_impl.write(b"use byteorder::{LittleEndian,WriteBytesExt};\n");
    file_packets_impl.write(b"use std::any::Any;\n");
    file_packets_impl.write(b"use std::convert::TryInto;\n\n");

    write_packet_parser(&mut file_packets_parser, &packets);
    write_packet_trait(&mut file_packets);
    for packet in packets {
        write_struct_definition(&mut file_packets, &packet.struct_def);
        write_struct_impl(&mut file_packets_impl, &packet.struct_def, Some(packet.id.clone()));
        write_packet_trait_impl(&mut file_packets_impl, &packet);
        write_debug_trait(&mut file_packets_print, &packet.struct_def, true);
        write_display_trait(&mut file_packets_print, &packet.struct_def, true);
    }

    for nested_struct in nested_structures {
        write_struct_definition(&mut file_packets, &nested_struct);
        write_struct_impl(&mut file_packets_impl, &nested_struct, None);
        write_debug_trait(&mut file_packets_print, &nested_struct, false);
        write_display_trait(&mut file_packets_print, &nested_struct, false);
    }
    write_unknown_packet(&mut file_packets);
}

fn write_file_header(file: &mut File) {
    file.write(b"// Generated by tools/packets_db/main.rs\n").unwrap();
    file.write(b"// Auto generated file do not edit manually\n\n");
}

fn write_packet_parser(file: &mut File, packets: &Vec<PacketStructDefinition>) {
    file.write(b"pub fn parse(buffer: &[u8]) -> Box<dyn Packet> {\n").unwrap();
    for packet in packets {
        let packet_id = packet_id(packet).replace("0x", "");
        let (first_byte, second_byte) = packet_id.split_at(2);
        file.write(format!("    if buffer[0] == 0x{} && buffer[1] == 0x{} {{\n", first_byte, second_byte).as_bytes());
        file.write(format!("        return Box::new({}::from(buffer));\n", packet.struct_def.name).as_bytes());
        file.write(b"    }\n");
    }
    file.write(b"    Box::new(PacketUnknown::from(buffer))\n");
    file.write(b"}\n\n").unwrap();
}

fn write_packet_trait(file: &mut File) {
    file.write(b"pub trait Packet {\n");
    file.write(b"    fn id(&self) -> &str;\n");
    file.write(b"    fn display(&self);\n");
    file.write(b"    fn debug(&self);\n");
    file.write(b"    fn pretty_debug(&self);\n");
    file.write(b"    fn raw(&self) -> &Vec<u8>;\n");
    file.write(b"    fn as_any(&self) -> &dyn Any;\n");
    file.write(b"    fn as_any_mut(&mut self) -> &mut dyn Any;\n");
    file.write(b"}\n\n");
}

fn write_packet_trait_impl(file: &mut File, packet: &PacketStructDefinition) {
    file.write(format!("impl Packet for {} {{\n", packet.struct_def.name).as_bytes());
    file.write(b"    fn id(&self) -> &str {\n");
    let id = packet_id(packet);
    file.write(format!("       \"{}\"\n", id).as_bytes());
    file.write(b"    }\n");
    file.write(b"    fn debug(&self) {\n");
    file.write(b"            println!(\"{:?}\", self)\n");
    file.write(b"    }\n");
    file.write(b"    fn display(&self) {\n");
    file.write(b"            println!(\"{}\", self)\n");
    file.write(b"    }\n");
    file.write(b"    fn pretty_debug(&self) {\n");
    file.write(b"            println!(\"{:#?}\", self)\n");
    file.write(b"    }\n");
    file.write(b"    fn raw(&self) -> &Vec<u8> {\n");
    file.write(b"            &self.raw\n");
    file.write(b"    }\n");
    file.write(b"    fn as_any(&self) -> &dyn Any{\n");
    file.write(b"        self\n");
    file.write(b"    }\n");
    file.write(b"    fn as_any_mut(&mut self) -> &mut dyn Any{\n");
    file.write(b"        self\n");
    file.write(b"    }\n");
    file.write(b"}\n\n");
}

fn write_debug_trait(file: &mut File, struct_definition: &StructDefinition, is_packet: bool) {
    file.write(format!("impl Debug for {} {{\n", struct_definition.name).as_bytes());
    file.write(b"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {\n");
    file.write(format!("        f.debug_struct(\"{}\")\n", struct_definition.name).as_bytes());
    if is_packet {
        file.write(b"            .field(\"id\", &self.id())\n");
    }
    for field in &struct_definition.fields {
        file.write(format!("            .field(\"{}{}\", &format!(\"{{:02X?}}\", &self.{}_raw))\n",
                           field.name,
                           format!("[{}, {}]", field.position, if field.length > -1 { (i32::from(field.position) + field.length).to_string() } else { "?".to_string() }),
                           field.name
        ).as_bytes());
    }
    file.write(b"        .finish()\n");
    file.write(b"    }\n");
    file.write(b"}\n\n");
}

fn write_display_trait(file: &mut File, struct_definition: &StructDefinition, is_packet: bool) {
    file.write(format!("impl Display for {} {{\n", struct_definition.name).as_bytes());
    file.write(b"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {\n");
    file.write(b"        let mut fields = Vec::new();\n");
    for field in &struct_definition.fields {
        let mut value_to_print = String::new();
        if field.data_type.name == "Array" {
            value_to_print = format!("&self.{}.pretty_output()", field.name);
        } else if field.data_type.name == "Vec" {
            value_to_print = format!("&self.{}.iter().map(|item| format!(\"\n  >{{}}\", item)).collect::<String>()", field.name);
        } else {
            value_to_print = format!("&self.{}", field.name);
        }
        file.write(format!("        fields.push(format!(\"{}{}{}: {{}}\", {}));\n",
                           field.name,
                           display_type(&field),
                           format!("[{}, {}]", field.position, if field.length > -1 { (i32::from(field.position) + field.length).to_string() } else { "?".to_string() }),
                           value_to_print
        ).as_bytes());
    }
    file.write(format!("        write!(f, \"{}\\n {{}}\", fields.join(\",\\n \"))\n", struct_definition.name).as_bytes());
    file.write(b"    }\n");
    file.write(b"}\n\n");
}

fn write_struct_definition(file: &mut File, struct_definition: &StructDefinition) {
    file.write(b"#[derive(Clone)]\n");
    file.write(format!("pub struct {} {{\n", struct_definition.name).as_bytes());
    file.write(b"    pub raw: Vec<u8>,\n");
    for field in &struct_definition.fields {
        file.write(format!("    pub {}: {},\n", field.name, field_type(field)).as_bytes());
        file.write(format!("    pub {}_raw: {},\n", field.name, field_type_raw(field)).as_bytes());
    }
    file.write(b"}\n\n");
}

fn write_struct_impl(file: &mut File, struct_definition: &StructDefinition, packet_id: Option<String>) {
    file.write(format!("impl {} {{\n", struct_definition.name).as_bytes());
    write_struct_from_method(file, struct_definition);
    write_struct_fill_raw_method(file, struct_definition);
    write_struct_setter_methods(file, struct_definition);
    write_struct_new_method(file, struct_definition, packet_id);
    file.write(b"}\n\n");
}

fn write_struct_from_method(file: &mut File, struct_definition: &StructDefinition) {
    file.write(format!("    pub fn from(buffer: &[u8]) -> {} {{\n", struct_definition.name).as_bytes());
    let field_with_vec = struct_definition.fields.iter().find(|field| field.data_type.name == "Vec");
    if field_with_vec.is_some() {
        write_vec_field(file, &field_with_vec.unwrap());
    }
    file.write(format!("        {} {{\n", struct_definition.name).as_bytes());
    file.write(b"            raw: buffer.to_vec(),\n");
    for field in &struct_definition.fields {
        if field.data_type.name == "Vec" {
            file.write(format!("            {}: vec_field.clone(),\n", field.name).as_bytes());
        } else if field.data_type.name == "Struct" {
            file.write(format!("            {}: {}::from(&buffer[{}..{}]),\n", field.name, field.complex_type.as_ref().unwrap(), field.position, field_length(field)).as_bytes());
        } else {
            file.write(format!("            {}: {},\n", field.name, struct_impl_field_value(field)).as_bytes());
        }
        if &field.data_type.name == "Vec" {
            file.write(format!("            {}_raw: vec_field.iter().map(|item| item.raw.clone()).collect::<Vec<Vec<u8>>>(),\n", field.name).as_bytes());
        } else if field.length > -1 {
            file.write(format!("            {}_raw: {{\n", field.name).as_bytes());
            file.write(format!("                let mut dst: [u8; {}] = [0u8; {}];\n", field.length, field.length).as_bytes());
            file.write(format!("                dst.clone_from_slice(&buffer[{}..{}]);\n", field.position, field.position + field.length as i16).as_bytes());
            file.write(b"                dst\n");
            file.write(b"            },\n");
        } else {
            file.write(format!("            {}_raw: buffer[{}..{}].to_vec(),\n", field.name, field.position, field_length(field)).as_bytes());
        }
    }
    file.write(b"        }\n");
    file.write(b"    }\n");
}

fn write_struct_fill_raw_method(file: &mut File, struct_definition: &StructDefinition) {
    file.write(b"    pub fn fill_raw(&mut self) {\n");
    file.write(b"    let mut wtr;\n");
    for field in &struct_definition.fields {
        file.write(field_serialization(field).as_bytes());
        file.write(b"\n");
    }
    file.write(b"        wtr = vec![];\n");
    for field in &struct_definition.fields {
        if field.data_type.name == "Vec" && field.complex_type.is_some() {
            file.write(format!("        self.{}.iter_mut().for_each(|item| wtr.append(&mut item.raw));\n", field.name).as_bytes());
        } else {
            file.write(format!("        wtr.append(&mut self.{}_raw.to_vec());\n", field.name).as_bytes());
        }
    }
    file.write(b"        self.raw = wtr;\n");
    file.write(b"    }\n");
}

fn write_struct_setter_methods(file: &mut File, struct_definition: &StructDefinition) {
    for field in &struct_definition.fields {
        file.write(format!("    pub fn set_{}(&mut self, value: {}) {{\n", field.name, field_type(field)).as_bytes());
        file.write(format!("        self.{} = value;\n", field.name).as_bytes());
        file.write(b"    }\n");
        file.write(format!("    pub fn set_{}_raw(&mut self, value: {}) {{\n", field.name, field_type_raw(field)).as_bytes());
        file.write(format!("        self.{}_raw = value;\n", field.name).as_bytes());
        file.write(b"    }\n");
    }
}

fn write_struct_new_method(file: &mut File, struct_definition: &StructDefinition, packet_id: Option<String>) {
    file.write(format!("    pub fn new() -> {} {{\n", struct_definition.name).as_bytes());
    file.write(format!("        {} {{\n", struct_definition.name).as_bytes());
    file.write(b"        raw: vec![],\n");
    for field in &struct_definition.fields {
        if field.name == "packet_id" && packet_id.is_some() {
            let id = packet_id.clone().unwrap().replace("0x", "");
            let (first_byte, second_byte) = id.split_at(2);
            let mut second_byte = second_byte;
            if second_byte == "" {
                second_byte = "0"
            }
            file.write(format!("        packet_id: i16::from_le_bytes([0x{}, 0x{}]),\n", first_byte, second_byte).as_bytes());
            file.write(format!("        packet_id_raw: [0x{}, 0x{}],\n", first_byte, second_byte).as_bytes());
        } else {
            file.write(field_default_value(field).as_bytes());
        }
    }
    file.write(b"        }\n");
    file.write(b"    }\n");
}

fn write_vec_field(file: &mut File, field: &StructField) {
    file.write(format!("        let iter_count = (&buffer.len() - {}) / {};\n", field.position, field.length).as_bytes());
    file.write(format!("        let mut vec_field: Vec<{}> = Vec::new();\n", field.complex_type.as_ref().unwrap()).as_bytes());
    file.write(b"        let mut i = 1;\n");
    file.write(b"        while i <= iter_count {\n");
    file.write(format!("            let start_pos = {} + ({} * (i - 1));\n", field.position, field.length).as_bytes());
    file.write(format!("            let end_pos = {} + {} * i;\n", field.position, field.length).as_bytes());
    file.write(format!("            vec_field.push({}::from(&buffer[start_pos..end_pos]));\n", field.complex_type.as_ref().unwrap()).as_bytes());
    file.write(b"            i += 1;\n");
    file.write(b"        }\n");
}

fn write_unknown_packet(file: &mut File) {
    file.write(b"#[derive(Debug)]\n");
    file.write(b"pub struct PacketUnknown {\n");
    file.write(b"    pub raw: Vec<u8>,\n");
    file.write(b"    pub packet_id: String,\n");
    file.write(b"}\n");
    file.write(b"impl Packet for PacketUnknown {\n");
    file.write(b"    fn id(&self) -> &str {\n");
    file.write(b"        self.packet_id.as_str()\n");
    file.write(b"    }\n");
    file.write(b"    fn debug(&self) {\n");
    file.write(b"            println!(\"{:?}\", self)\n");
    file.write(b"    }\n");
    file.write(b"    fn display(&self) {\n");
    file.write(b"            self.debug()\n");
    file.write(b"    }\n");
    file.write(b"    fn pretty_debug(&self) {\n");
    file.write(b"            self.debug()\n");
    file.write(b"    }\n");
    file.write(b"    fn raw(&self) -> &Vec<u8> {\n");
    file.write(b"            &self.raw\n");
    file.write(b"    }\n");
    file.write(b"    fn as_any(&self) -> &dyn Any{\n");
    file.write(b"        self\n");
    file.write(b"    }\n");
    file.write(b"    fn as_any_mut(&mut self) -> &mut dyn Any{\n");
    file.write(b"        self\n");
    file.write(b"    }\n");
    file.write(b"}\n");
    file.write(b"impl PacketUnknown {\n");
    file.write(b"    pub fn from(buffer: &[u8]) -> PacketUnknown {\n");
    file.write(b"        PacketUnknown { raw: buffer.to_vec(), packet_id: format!(\"0x{:02X?}{:02X?}\", buffer[0], buffer[1])}\n");
    file.write(b"    }\n");
    file.write(b"}\n");
}


fn struct_impl_field_value(field: &StructField) -> String {
    match field.data_type.name.as_str() {
        "char" => {
            format!("buffer[{}] as char", field.position)
        }
        "u8" => {
            format!("u8::from_le_bytes([buffer[{}]])", field.position)
        }
        "i8" => {
            format!("i8::from_le_bytes([buffer[{}]])", field.position)
        }
        "u16" => {
            format!("u16::from_le_bytes([buffer[{}], buffer[{}]])", field.position, field.position + 1)
        }
        "i16" => {
            format!("i16::from_le_bytes([buffer[{}], buffer[{}]])", field.position, field.position + 1)
        }
        "u32" => {
            format!("u32::from_le_bytes([buffer[{}], buffer[{}], buffer[{}], buffer[{}]])", field.position, field.position + 1, field.position + 2, field.position + 3)
        }
        "i32" => {
            format!("i32::from_le_bytes([buffer[{}], buffer[{}], buffer[{}], buffer[{}]])", field.position, field.position + 1, field.position + 2, field.position + 3)
        }
        "u64" => {
            format!("u64::from_le_bytes([buffer[{}], buffer[{}], buffer[{}], buffer[{}], buffer[{}], buffer[{}], buffer[{}], buffer[{}]])", field.position, field.position + 1, field.position + 2, field.position + 3, field.position + 4, field.position + 5, field.position + 6, field.position + 7)
        }
        "i64" => {
            format!("i64::from_le_bytes([buffer[{}], buffer[{}], buffer[{}], buffer[{}], buffer[{}], buffer[{}], buffer[{}], buffer[{}]])", field.position, field.position + 1, field.position + 2, field.position + 3, field.position + 4, field.position + 5, field.position + 6, field.position + 7)
        }
        "bool" => {
            format!("buffer[{}] == 1", field.position)
        }
        "String" => {
            format!("String::from_utf8_lossy(&buffer[{}..{}]).to_string()", field.position, field_length(field))
        }
        "Array" => {
            let mut array_block = " {\n".to_string();
            let length = &field.length;
            if field.sub_type.is_some() {
                let sub_type_name = &field.sub_type.unwrap().name;
                array_block = format!("{}                let mut dst: [{}; {}] = [0 as {}; {}];\n", array_block, sub_type_name, length, sub_type_name, length);
                array_block = format!("{}                for (index, byte) in buffer[{}..{}].iter().enumerate() {{\n", array_block, field.position, field.position + field.length as i16);
                array_block = format!("{}                    dst[index] = *byte as {};\n", array_block, sub_type_name);
                array_block = format!("{}                }}\n", array_block);
            } else if field.length > -1 {
                array_block = format!("{}                let mut dst: [u8; {}] = [0; {}];\n", array_block, length, length);
                array_block = format!("{}                dst.clone_from_slice(&buffer[{}..{}]);\n", array_block, field.position, field.position + field.length as i16);
            } else {
                array_block = format!("{}                let dst: Vec<u8> = buffer[{}..buffer.len()].to_vec();\n", array_block, field.position);
            }
            array_block = format!("{}                dst\n            }}", array_block);
            array_block
        }
        _ => {
            format!("\"found unknown type {} for field {}. this won't compile!\"", field.data_type.name, field.name)
        }
    }
}

fn field_serialization(field: &StructField) -> String {
    let mut res = String::new();
    match field.data_type.name.as_str() {
        "char" => {
            res = "        wtr = vec![];\n".to_string();
            res = format!("{}        wtr.write_u8(self.{}).unwrap();\n", res, field.name);
            res = format!("{}        self.{}_raw = wtr.try_into().unwrap();", res, field.name);
            res
        }
        "bool" => {
            res = "        wtr = vec![];\n".to_string();
            res = format!("{}        wtr.write_u8(self.{} as u8).unwrap();\n", res, field.name);
            res = format!("{}        self.{}_raw = wtr.try_into().unwrap();", res, field.name);
            res
        }
        "u8" | "i8" => {
            res = "        wtr = vec![];\n".to_string();
            res = format!("{}        wtr.write_{}(self.{}).unwrap();\n", res, field.data_type.name, field.name);
            res = format!("{}        self.{}_raw = wtr.try_into().unwrap();", res, field.name);
            res
        }
        "u16" | "i16" | "u32" | "i32" | "u64" | "i64" => {
            res = "        wtr = vec![];\n".to_string();
            res = format!("{}        wtr.write_{}::<LittleEndian>(self.{}).unwrap();\n", res, field.data_type.name, field.name);
            res = format!("{}        self.{}_raw = wtr.try_into().unwrap();", res, field.name);
            res
        }
        "String" => {
            format!("        self.{}_raw = self.{}.as_bytes().to_vec();", field.name, field.name)
        }
        "Array" => {
            if field.sub_type.is_some() {
                res = "        wtr = vec![];\n".to_string();
                let sub_type = field.sub_type.unwrap();
                if sub_type.name == "u8" || sub_type.name == "i8" {
                    res = format!("{}        for item in self.{} {{\n", res, field.name);
                    res = format!("{}            wtr.write_{}(item).unwrap();\n", res, sub_type.name);
                    res = format!("{}        }}\n", res);
                } else if sub_type.name == "char" {
                    res = format!("{}        for item in self.{} {{\n", res, field.name);
                    res = format!("{}            wtr.write_u8(item as u8 ).unwrap();\n", res);
                    res = format!("{}        }}\n", res);
                } else {
                    res = format!("{}        for item in self.{} {{\n", res, field.name);
                    res = format!("{}            wtr.write_{}::<LittleEndian>(item).unwrap();\n", res, sub_type.name);
                    res = format!("{}        }}\n", res);
                }
                res = format!("{}        self.{}_raw = wtr.try_into().unwrap();", res, field.name);
                res
            } else {
                res
            }
        }
        "Vec" => {
            if field.complex_type.is_some() {
                res = format!("        self.{}_raw = {{\n", field.name);
                res = format!("{}            self.{}.iter_mut().for_each(|item| item.fill_raw());\n", res, field.name);
                res = format!("{}            self.{}.iter().map(|item| item.raw.clone()).collect()\n", res, field.name);
                res = format!("{}      }};\n", res);
                res
            } else {
                format!("\"found unknown type {} for field {}. this won't compile!\"", field.data_type.name, field.name)
            }
        }
        "Struct" => {
            if field.complex_type.is_some() {
                res = format!("        self.{}.fill_raw();\n", field.name);
                if field.length > -1 {
                    res = format!("{}        self.{}_raw = self.{}.clone().raw.try_into().unwrap();\n", res, field.name, field.name);
                } else {
                    res = format!("{}        self.{}_raw = self.{}.clone().raw;\n", res, field.name, field.name);
                }
                res
            } else {
                format!("\"found unknown type {} for field {}. this won't compile!\"", field.data_type.name, field.name)
            }
        }
        _ => {
            format!("\"found unknown type {} for field {}. this won't compile!\"", field.data_type.name, field.name)
        }
    }
}

fn field_length(field: &StructField) -> String {
    if field.length > -1 { (field.position + field.length as i16).to_string() } else { "buffer.len()".to_string() }
}

fn packet_id(packet: &PacketStructDefinition) -> String {
    let mut id = packet.id.clone();
    if packet.id.len() == 4 {
        id = format!("{:0<6}", packet.id);
    } else if packet.id.len() == 5 {
        id = packet.id.replace("0x", "0x0");
    }
    id
}

fn display_type(field: &StructField) -> String {
    if field.data_type.name == "Array" && field.sub_type.is_some() {
        let sub_type = field.sub_type.unwrap();
        return format!("({}[] as {}[])", sub_type.cname, sub_type.name);
    }

    format!("({} as {})", field.data_type.cname, field.data_type.name)
}

fn field_type(field: &StructField) -> String {
    if field.data_type.name == "Vec" {
        format!("Vec<{}>", field.complex_type.as_ref().unwrap())
    } else if field.data_type.name == "Struct" {
        field.complex_type.as_ref().unwrap().to_string()
    } else if field.data_type.name == "Array" {
        if field.sub_type.is_some() {
            format!("[{}; {}]", &field.sub_type.unwrap().name, field.length)
        } else {
            "Vec<u8>".to_string()
        }
    } else {
        format!("{}", field.data_type.name)
    }
}

fn field_type_raw(field: &StructField) -> String {
    if field.data_type.name == "Vec" {
        "Vec<Vec<u8>>".to_string()
    } else if field.length > -1 {
        format!("[u8; {}]", field.length)
    } else {
        "Vec<u8>".to_string()
    }
}

fn field_default_value(field: &StructField) -> String {
    let mut res = String::new();
    match field.data_type.name.as_str() {
        "char" => {
            res = format!("{}        {}: '',\n", res, field.name);
            res = format!("{}        {}_raw: 0,\n", res, field.name);
            res
        }
        "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" => {
            res = format!("{}        {}: 0,\n", res, field.name);
            res = format!("{}        {}_raw: [0; {}],\n", res, field.name, field.length);
            res
        }
        "bool" => {
            res = format!("{}        {}: false,\n", res, field.name);
            res = format!("{}        {}_raw: [0; 1],\n", res, field.name);
            res
        }
        "String" => {
            res = format!("{}        {}: String::new(),\n", res, field.name);
            res = format!("{}        {}_raw: vec![],\n", res, field.name);
            res
        }
        "Array" => {
            if field.length > -1 {
                let mut value = "0";
                if field.sub_type.unwrap().name == "char" {
                    value = "0 as char";
                }
                res = format!("{}        {}: [{}; {}],\n", res, field.name, value, field.length);
                res = format!("{}        {}_raw: [0; {}],\n", res, field.name, field.length);
                res
            } else {
                res = format!("{}        {}: vec![],\n", res, field.name);
                res = format!("{}        {}_raw: vec![],\n", res, field.name);
                res
            }
        }
        "Vec" => {
            res = format!("{}        {}: vec![],\n", res, field.name);
            res = format!("{}        {}_raw: vec![],\n", res, field.name);
            res
        }
        "Struct" => {
            res = format!("{}        {}: {}::new(),\n", res, field.name, field.complex_type.as_ref().unwrap());
            if field.length > -1 {
                res = format!("{}        {}_raw: [0; {}],\n", res, field.name, field.length);
            } else {
                res = format!("{}        {}_raw: vec![],\n", res, field.name);
            }
            res
        }
        _ => {
            panic!("\"found unknown type {} for field {}. this won't compile!\"", field.data_type.name, field.name)
        }
    }
}
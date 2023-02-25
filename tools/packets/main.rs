mod packet_struct_generator;
mod packet_db_parser;

use std::fmt::{Debug};
use crate::packet_struct_generator::write_packets_struct;
use crate::packet_db_parser::parse;
use std::path::Path;

#[derive(Debug)]
pub struct PacketStructDefinition<'a> {
    pub ids: Vec<PacketId>,
    pub struct_def: StructDefinition<'a>,
}

#[derive(Clone, Debug)]
pub struct PacketId {
    pub id: String,
    pub packetver: Option<u32>,
}

#[derive(Debug)]
pub struct StructDefinition<'a> {
    pub name: String,
    pub current_field_position: i16,
    pub fields: Vec<StructField<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructField<'a> {
    pub name: String,
    pub position: i16,
    pub length: i16,
    pub data_type: &'a Type,
    pub complex_type: Option<String>,
    pub sub_type: Option<&'a Type>,
    pub condition: Option<Condition>,
}
#[derive(Debug, Clone)]
pub enum Condition {
    GTE(u32),
    GT(u32),
    LTE(u32),
    LT(u32),
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub cname: String,
    pub length: Option<i16>,
}

impl <'a> StructDefinition<'a> {
    pub fn increment_current_field_position(&mut self, last_field_length: i16) {
        self.current_field_position += last_field_length;
    }
}

/*
TODO make a cli from this.
 */
fn main() {
    let packet_db_path = Path::new("tools/packets/packets_db");
    let output_path = Path::new("lib/packets/src");
    let (packets, nested_structs) = parse(packet_db_path);
    write_packets_struct(packets, &nested_structs, output_path);
}

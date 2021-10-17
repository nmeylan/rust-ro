mod packet_struct_generator;
mod packet_db_parser;


use std::fmt::{Debug};


use crate::packet_struct_generator::write_packets_struct;
use crate::packet_db_parser::parse;
use std::path::Path;

#[derive(Debug)]
pub struct PacketStructDefinition<'a> {
    pub id: String,
    pub struct_def: StructDefinition<'a>,
}

#[derive(Debug)]
pub struct StructDefinition<'a> {
    pub name: String,
    pub fields: Vec<StructField<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructField<'a> {
    pub name: String,
    pub position: i16,
    pub length: i32,
    pub data_type: &'a Type,
    pub complex_type: Option<String>,
    pub sub_type: Option<&'a Type>,
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub cname: String,
    pub length: Option<i32>,
}

/*
TODO make a cli from this.
 */
fn main() {
    let packet_db_path = Path::new("server/tools/packets/packets_db");
    let output_path = Path::new("server/src/packets");
    let (packets, nested_structs) = parse(packet_db_path);
    write_packets_struct(packets, &nested_structs, output_path);
}

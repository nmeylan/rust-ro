mod packet_struct_generator;
mod packet_db_parser;

use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::collections::HashMap;
use std::sync::{Mutex};
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use maplit::hashmap;
use lazy_static::lazy_static;
use regex::{Regex, Captures};
use std::fmt::{Debug, Formatter, Alignment};
use std::ops::Deref;
use std::borrow::{BorrowMut, Borrow};
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

fn main() {
    let packet_db_path = Path::new("tools/packets/packets_db");
    let output_path = Path::new("src/packets");
    let (packets, nested_structs) = parse(packet_db_path);
    write_packets_struct(packets, &nested_structs, output_path);
}

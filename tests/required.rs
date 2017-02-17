//! Automatically generated rust module for 'required.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate quick_protobuf;

use std::io::Write;
use quick_protobuf::{MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;

// generated from:
//
// package a.b.c;
//
// enum Resultish {
//   Okay = 0;
//   Fail = 1;
// }
//
// message Foo {
//   required Resultish result = 1;
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Resultish {
    Okay = 0,
    Fail = 1,
}

impl Default for Resultish {
    fn default() -> Self {
        Resultish::Okay
    }
}

impl From<i32> for Resultish {
    fn from(i: i32) -> Self {
        match i {
            0 => Resultish::Okay,
            1 => Resultish::Fail,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Foo {
    pub result: Option<Resultish>,
}

impl Foo {
    pub fn from_reader(r: &mut BytesReader, bytes: &[u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.result = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Foo {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[test]
#[should_panic]
fn test_it_serializes_without_required_field() {
    let mut out = std::io::Cursor::new(Vec::new());
    Foo { result: None }.write_message(&mut Writer::new(&mut out)).unwrap();
}

#[test]
#[should_panic]
fn test_it_deserializes_empty() {
    let input = Vec::new();
    let _ = Foo::from_reader(&mut BytesReader::from_bytes(&input), &input).unwrap();
}

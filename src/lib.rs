#[macro_use]
extern crate nom;

pub mod data;
pub mod parser;

use data::{Transaction};
use parser::{parse_file};
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn parse_ledger_file(filename : &str) -> Result<Vec<Transaction>, &'static str> {
    let mut f = match File::open(filename) {
        Err(_) => return Err("File Open Error"),
        Ok(f) => f
    };

    let mut buffer = Vec::new();
    
    match f.read_to_end(&mut buffer) {
        Err(_) => return Err("File Read Error"),
        _ => ()
    };
    
    match parse_file(buffer.as_slice()) {
        nom::IResult::Done(_, t) => return Ok(t),
        nom::IResult::Error(_) => return Err("Parse Error"),
        nom::IResult::Incomplete(_) => return Err("Parse Error")
    }
}

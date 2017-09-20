extern crate wasmparser;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::str;
use wasmparser::WasmDecoder;
use wasmparser::Parser;
use wasmparser::ParserState;

fn main() {
    let ref buf: Vec<u8> = read_wasm("tanks.wasm").unwrap();
    let mut parser = Parser::new(buf);
    let mut operator_counter = 0;
    let mut function_counter = 0;
    loop {
        let state = parser.read();
        match *state {
            ParserState::EndWasm => break,
            ParserState::Error(err) => panic!("Error: {:?}", err),
            ParserState::BeginFunctionBody { .. } => function_counter += 1,
            ParserState::CodeOperator(_) => operator_counter += 1,
            _ => (), // println!("{:?}", state),
        }
    }
    println!("Functions = {}", function_counter);
    println!("Operators = {}", operator_counter);
}

fn read_wasm(file: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut f = File::open(file)?;
    f.read_to_end(&mut data)?;
    Ok(data)
}

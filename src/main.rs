use virtual_machine::code_writer;
use virtual_machine::parser;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn read_lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Failed to read line"))
        .collect()
}

fn main() {
    let commands = read_lines_from_file("../../projects/07/MemoryAccess/StaticTest/StaticTest.vm");
    let mut parser = parser::Parser::new(commands);
    let mut code_writer = code_writer::CodeWriter::new("StaticTest.vm".to_string());
    while parser.has_more_commands {
        parser.advance();
        match parser.command_type {
            Some(parser::CommandType::ARITHMETIC) => {
                code_writer.run_arichmetic_command(parser.arg1.as_deref().unwrap())
            }
            Some(parser::CommandType::PUSH) => code_writer.push(
                parser.arg1.as_deref().unwrap(),
                parser.arg2.as_deref().unwrap(),
            ),
            _ => (),
        }
    }
    code_writer.output("./output.asm");
}

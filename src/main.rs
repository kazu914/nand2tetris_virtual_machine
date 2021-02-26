use virtual_machine::code_writer;
use virtual_machine::parser;

use std::{
    env,
    fs::File,
    io,
    io::{prelude::*, BufReader},
    process,
};

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Filename is not provided");
        }
        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

fn read_lines_from_file(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename);

    let file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .collect())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let commands = read_lines_from_file(&config.filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1)
    });

    let mut parser = parser::Parser::new(commands);
    let mut code_writer = code_writer::CodeWriter::new(config.filename.clone());
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
            Some(parser::CommandType::POP) => code_writer.pop(
                parser.arg1.as_deref().unwrap(),
                parser.arg2.as_deref().unwrap(),
            ),
            Some(parser::CommandType::LABEL) => {
                code_writer.write_label(parser.arg1.as_deref().unwrap())
            }
            Some(parser::CommandType::GOTO) => {
                code_writer.write_go_to(parser.arg1.as_deref().unwrap())
            }
            Some(parser::CommandType::IF) => {
                code_writer.write_if_go_to(parser.arg1.as_deref().unwrap())
            }
            Some(parser::CommandType::CALL) => code_writer.write_call(
                parser.arg1.as_deref().unwrap(),
                parser.arg2.as_deref().unwrap(),
            ),
            Some(parser::CommandType::FUNCTION) => code_writer.write_function(
                parser.arg1.as_deref().unwrap(),
                parser.arg2.as_deref().unwrap(),
            ),
            _ => (),
        }
    }
    code_writer.output(&config.filename.replace(".vm", ".asm"));
}

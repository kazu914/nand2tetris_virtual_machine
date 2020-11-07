use virtual_machine::code_writer;
use virtual_machine::parser;

fn main() {
    let commands = vec![
        "push constant 7".to_string(),
        "push constant 8".to_string(),
        "add".to_string(),
    ];
    let mut parser = parser::Parser::new(commands);
    let mut code_writer = code_writer::CodeWriter::new("tmp.asm".to_string());
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
    code_writer.output();
}

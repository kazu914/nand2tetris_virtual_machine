pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
}

pub struct Parser {
    pub has_more_commands: bool,
    pub command_type: Option<CommandType>,
    pub arg1: Option<String>,
    pub arg2: Option<String>,
    commands: Vec<String>,
    index: usize,
}

impl Parser {
    pub fn new(commands: Vec<String>) -> Parser {
        let has_more_commands = commands.len() > 0;
        Parser {
            commands,
            index: 0,
            has_more_commands,
            command_type: None,
            arg1: None,
            arg2: None,
        }
    }
}

pub fn main() {
    let commands = vec!["push constant 15".to_string()];
    let parer = Parser::new(commands);
    println!("main")
}

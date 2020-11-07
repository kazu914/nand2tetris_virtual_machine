#[derive(Debug, PartialEq)]
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
        let actual_commands = Parser::remove_unnecessary_parts(commands);
        Parser {
            has_more_commands: actual_commands.len() > 0,
            commands: actual_commands,
            index: 0,
            command_type: None,
            arg1: None,
            arg2: None,
        }
    }

    pub fn advance(&mut self) {
        if self.has_more_commands {
            self.clear();
            let command = self.commands[self.index].as_str();
            let command_type = Parser::classify_command(command);
            self.command_type = Some(command_type);
            self.index += 1;
            self.has_more_commands = self.commands.len() > self.index;
        }
    }

    fn clear(&mut self) {
        self.command_type = None;
        self.arg1 = None;
        self.arg2 = None;
    }

    fn classify_command(command: &str) -> CommandType {
        let firtst_word = command.split(" ").collect::<Vec<&str>>()[0];
        match firtst_word {
            "add" => CommandType::ARITHMETIC,
            "sub" => CommandType::ARITHMETIC,
            "neg" => CommandType::ARITHMETIC,
            "eq" => CommandType::ARITHMETIC,
            "gt" => CommandType::ARITHMETIC,
            "lt" => CommandType::ARITHMETIC,
            "and" => CommandType::ARITHMETIC,
            "or" => CommandType::ARITHMETIC,
            "not" => CommandType::ARITHMETIC,
            "push" => CommandType::PUSH,
            "pop" => CommandType::POP,
            "label" => CommandType::LABEL,
            "goto" => CommandType::GOTO,
            "if-goto" => CommandType::IF,
            "function" => CommandType::FUNCTION,
            "return" => CommandType::RETURN,
            "call" => CommandType::CALL,

            _ => panic!("Invalid command"),
        }
    }

    fn remove_comments(command: String) -> String {
        command.split("//").collect::<Vec<&str>>()[0]
            .trim()
            .to_string()
    }

    fn remove_unnecessary_parts(original_commands: Vec<String>) -> Vec<String> {
        let mut new_commands = Vec::new();
        for command in original_commands {
            let flag = command.trim().chars().nth(0);
            match flag {
                Some('/') => (),
                None => (),
                Some(_) => new_commands.push(Parser::remove_comments(command)),
            }
        }

        new_commands
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parer_remove_comments() {
        let command = "   push constant 7  // this is comment".to_string();
        assert_eq!(Parser::remove_comments(command), "push constant 7");
    }

    #[test]
    fn parer_remove_unnecessary_parts() {
        let original_commands = vec![
            "//this is comment line".to_string(),
            "push constant 7 // here also comment".to_string(),
            "   add    //whitespace should be trimmed".to_string(),
        ];

        let new_commands = vec!["push constant 7".to_string(), "add".to_string()];
        assert_eq!(
            Parser::remove_unnecessary_parts(original_commands),
            new_commands
        );
    }
    #[test]
    fn parser_new() {
        let original_commands = vec![
            "//this is comment line".to_string(),
            "push constant 7 // here also comment".to_string(),
            "   add    //whitespace should be trimmed".to_string(),
        ];

        let new_commands = vec!["push constant 7".to_string(), "add".to_string()];
        let parer = Parser::new(original_commands);
        assert_eq!(parer.commands, new_commands);
        assert_eq!(parer.has_more_commands, true);
        assert_eq!(parer.index, 0);
        assert_eq!(parer.command_type, None);
        assert_eq!(parer.arg1, None);
        assert_eq!(parer.arg2, None);
    }

    #[test]
    fn parer_advance() {
        let original_commands = vec![
            "//this is comment line".to_string(),
            "push constant 7 // here also comment".to_string(),
            "   add    //whitespace should be trimmed".to_string(),
        ];
        let mut parer = Parser::new(original_commands);
        parer.advance();

        assert_eq!(parer.index, 1);
        assert_eq!(parer.command_type, Some(CommandType::PUSH));

        parer.advance();
        assert_eq!(parer.index, 2);
        assert_eq!(parer.command_type, Some(CommandType::ARITHMETIC));
    }

    #[test]
    fn parer_classify_command() {
        assert_eq!(Parser::classify_command("add"), CommandType::ARITHMETIC);
        assert_eq!(
            Parser::classify_command("push constant 7"),
            CommandType::PUSH
        );
        assert_eq!(Parser::classify_command("pop constant 7"), CommandType::POP);
        assert_eq!(Parser::classify_command("label"), CommandType::LABEL);
    }

    #[test]
    #[should_panic(expected = "Invalid command")]
    fn parer_classify_command_should_panic_with_invalid_command() {
        Parser::classify_command("invalid command");
    }
}

pub fn main() {
    let commands = vec!["push constant 15".to_string()];
    let parer = Parser::new(commands);
    println!("main")
}

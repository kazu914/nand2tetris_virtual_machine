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
    fn test_remove_comments() {
        let command = "   push constant 7  // this is comment".to_string();
        assert_eq!(Parser::remove_comments(command), "push constant 7");
    }

    #[test]
    fn test_remove_unnecessary_parts() {
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
}

pub fn main() {
    let commands = vec!["push constant 15".to_string()];
    let parer = Parser::new(commands);
    println!("main")
}

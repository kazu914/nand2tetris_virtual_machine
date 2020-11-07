#[derive(Debug, PartialEq)]
pub enum Segment {
    ARGUMENT,
    LOCAL,
    STATIC,
    CONSTANT,
    THIS,
    THAT,
    POINTER,
    TEMP,
}

impl Segment {
    pub fn from_str(s: &str) -> Option<Segment> {
        match s {
            "argument" => Some(Segment::ARGUMENT),
            "local" => Some(Segment::LOCAL),
            "static" => Some(Segment::STATIC),
            "constant" => Some(Segment::CONSTANT),
            "this" => Some(Segment::THIS),
            "that" => Some(Segment::THAT),
            "pointer" => Some(Segment::POINTER),
            "temp" => Some(Segment::TEMP),
            _ => panic!("Invalid Segment"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ArithmeticCommand {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}

impl ArithmeticCommand {
    pub fn from_str(s: &str) -> Option<ArithmeticCommand> {
        match s {
            "add" => Some(ArithmeticCommand::ADD),
            "sub" => Some(ArithmeticCommand::SUB),
            "neg" => Some(ArithmeticCommand::NEG),
            "eq" => Some(ArithmeticCommand::EQ),
            "gt" => Some(ArithmeticCommand::GT),
            "lt" => Some(ArithmeticCommand::LT),
            "and" => Some(ArithmeticCommand::AND),
            "or" => Some(ArithmeticCommand::OR),
            "not" => Some(ArithmeticCommand::NOT),
            _ => panic!("Invalid ArithmeticCommand"),
        }
    }
}

pub struct CodeWriter {
    file_name: String,
    generated_code: Vec<String>,
}

impl CodeWriter {
    pub fn new(file_name: String) -> CodeWriter {
        CodeWriter {
            file_name,
            generated_code: vec![],
        }
    }

    pub fn push(&mut self, segment: &str, index: &str) {
        match Segment::from_str(segment) {
            Some(Segment::CONSTANT) => {
                let mut commands = CodeWriter::push_constant(index);
                self.generated_code.append(&mut commands)
            }
            _ => (),
        }
    }

    pub fn run_arichmetic_command(&mut self, arithmetic_command: &str) {
        match ArithmeticCommand::from_str(arithmetic_command) {
            Some(ArithmeticCommand::ADD) => {
                let mut commands = CodeWriter::add();
                self.generated_code.append(&mut commands)
            }
            _ => (),
        }
    }

    fn push_constant(constant: &str) -> Vec<String> {
        vec![
            format!("@{}", constant),
            "D=A".to_string(),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ]
    }

    fn add() -> Vec<String> {
        vec![
            "@SP".to_string(),
            "M=M-1".to_string(),
            "AD=M".to_string(),
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "M=M+D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn push_constant() {
        let expected_result = vec![
            "@7".to_string(),
            "D=A".to_string(),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ];
        let result = CodeWriter::push_constant("7");
        assert_eq!(result, expected_result)
    }
}

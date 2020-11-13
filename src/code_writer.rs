use std::{fs::OpenOptions, io::prelude::*};

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
    symbol_count: usize,
}

impl CodeWriter {
    pub fn new(file_name: String) -> CodeWriter {
        CodeWriter {
            file_name,
            generated_code: vec![],
            symbol_count: 0,
        }
    }

    pub fn output(&self) {
        println!("{:#?}", self.generated_code);
        let mut output = OpenOptions::new()
            .create(true)
            .write(true)
            .open("/home/nomura/nand2tetris/projects/07/StackArithmetic/SimpleAdd/SimpleAdd.asm")
            .unwrap();
        for line in &self.generated_code {
            writeln!(output, "{}", line).unwrap();
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
        use self::ArithmeticCommand::*;
        let mut new_code = match ArithmeticCommand::from_str(arithmetic_command) {
            Some(ADD) => CodeWriter::add(),
            Some(SUB) => CodeWriter::sub(),
            Some(EQ) => {
                self.symbol_count += 1;
                CodeWriter::eq(&self.symbol_count)
            }
            Some(GT) => {
                self.symbol_count += 1;
                CodeWriter::gt(&self.symbol_count)
            }
            Some(LT) => {
                self.symbol_count += 1;
                CodeWriter::lt(&self.symbol_count)
            }
            _ => return,
        };
        self.generated_code.append(&mut new_code);
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
        CodeWriter::make_two_operands_code("+")
    }

    fn sub() -> Vec<String> {
        CodeWriter::make_two_operands_code("-")
    }

    fn make_two_operands_code(operator: &str) -> Vec<String> {
        vec![
            "@SP".to_string(),
            "M=M-1".to_string(),
            "A=M".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M-1".to_string(),
            "A=M".to_string(),
            format!("M=M{}D", operator),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ]
    }

    fn eq(symbol_count: &usize) -> Vec<String> {
        CodeWriter::make_condition_code(symbol_count, "JEQ")
    }

    fn gt(symbol_count: &usize) -> Vec<String> {
        CodeWriter::make_condition_code(symbol_count, "JGT")
    }

    fn lt(symbol_count: &usize) -> Vec<String> {
        CodeWriter::make_condition_code(symbol_count, "JLT")
    }

    fn make_condition_code(symbol_count: &usize, condition: &str) -> Vec<String> {
        vec![
            "@SP".to_string(),
            "M=M-1".to_string(),
            "A=M".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M-1".to_string(),
            "A=M".to_string(),
            "MD=M-D".to_string(), // M=x, D=y
            format!("@IF_CONDITION.{}", symbol_count),
            format!("D;{}", condition),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=0".to_string(),
            format!("@IF_CONDITION.{}.FINAL", symbol_count),
            "0;JMP".to_string(),
            format!("(IF_CONDITION.{})", symbol_count),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=-1".to_string(),
            format!("(IF_CONDITION.{}.FINAL)", symbol_count),
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

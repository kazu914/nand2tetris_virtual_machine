use std::{fs::OpenOptions, io::prelude::*};
mod arithmetic_code_generator;
mod arithmetic_command;
mod pop_code_generator;
mod push_code_generator;
mod segment;

const POINTER_BASE_ADDRESS: &str = "3";
const TEMP_BASE_ADDRESS: &str = "5";

pub struct CodeWriter {
    file_name: String,
    generated_code: Vec<String>,
    symbol_count: usize,
    function_name_stack: Vec<String>,
}

impl CodeWriter {
    pub fn new(file_name: String) -> CodeWriter {
        CodeWriter {
            file_name,
            generated_code: vec![],
            symbol_count: 0,
            function_name_stack: vec!["null".to_string()],
        }
    }

    pub fn output(&self, file_name: &str) {
        println!("{:#?}", self.generated_code);
        let mut output = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)
            .unwrap();
        for line in &self.generated_code {
            writeln!(output, "{}", line).unwrap();
        }
    }

    pub fn push(&mut self, segment: &str, index: &str) {
        use segment::Segment;
        let mut new_code = match Segment::from_str(segment) {
            Some(Segment::CONSTANT) => push_code_generator::push_constant(index),
            Some(Segment::LOCAL)
            | Some(Segment::ARGUMENT)
            | Some(Segment::THIS)
            | Some(Segment::THAT) => push_code_generator::push_segment(
                index,
                Segment::to_register_alias_str(segment).as_str(),
            ),
            Some(Segment::POINTER) => {
                push_code_generator::push_pointer_and_temp(index, POINTER_BASE_ADDRESS)
            }
            Some(Segment::TEMP) => {
                push_code_generator::push_pointer_and_temp(index, TEMP_BASE_ADDRESS)
            }
            Some(Segment::STATIC) => self.push_static(index),
            _ => return,
        };
        self.generated_code.append(&mut new_code);
    }
    pub fn pop(&mut self, segment: &str, index: &str) {
        use segment::Segment;
        let mut new_code = match Segment::from_str(segment) {
            Some(Segment::LOCAL)
            | Some(Segment::ARGUMENT)
            | Some(Segment::THIS)
            | Some(Segment::THAT) => pop_code_generator::pop_segment(
                index,
                Segment::to_register_alias_str(segment).as_str(),
            ),
            Some(Segment::POINTER) => {
                pop_code_generator::pop_pointer_and_temp(index, POINTER_BASE_ADDRESS)
            }
            Some(Segment::TEMP) => {
                pop_code_generator::pop_pointer_and_temp(index, TEMP_BASE_ADDRESS)
            }
            Some(Segment::STATIC) => self.pop_static(index),
            _ => return,
        };
        self.generated_code.append(&mut new_code);
    }

    pub fn write_label(&mut self, label_name: &str) {
        let mut new_code = vec![format!(
            "{}${}",
            self.function_name_stack.last().unwrap(),
            label_name
        )];
        self.generated_code.append(&mut new_code)
    }

    pub fn run_arichmetic_command(&mut self, arithmetic_command: &str) {
        use arithmetic_command::{ArithmeticCommand, ArithmeticCommand::*};
        let mut new_code = match ArithmeticCommand::from_str(arithmetic_command) {
            Some(ADD) => arithmetic_code_generator::add(),
            Some(SUB) => arithmetic_code_generator::sub(),
            Some(NEG) => arithmetic_code_generator::neg(),
            Some(EQ) => {
                self.symbol_count += 1;
                arithmetic_code_generator::eq(&self.symbol_count)
            }
            Some(GT) => {
                self.symbol_count += 1;
                arithmetic_code_generator::gt(&self.symbol_count)
            }
            Some(LT) => {
                self.symbol_count += 1;
                arithmetic_code_generator::lt(&self.symbol_count)
            }
            Some(AND) => arithmetic_code_generator::and(),
            Some(OR) => arithmetic_code_generator::or(),
            Some(NOT) => arithmetic_code_generator::not(),
            _ => return,
        };
        self.generated_code.append(&mut new_code);
    }

    fn push_static(&self, index: &str) -> Vec<String> {
        let constant_name = CodeWriter::camel_case_filename_without_extention(&self.file_name);
        let mut res = vec![format!("@{}.{}", constant_name, index), "D=M".to_string()];
        res.append(&mut push_code_generator::generate_push_d_to_sp_code());
        res
    }

    fn pop_static(&self, index: &str) -> Vec<String> {
        let constant_name = CodeWriter::camel_case_filename_without_extention(&self.file_name);
        let mut res = vec![
            format!("@{}.{}", constant_name, index),
            "D=A".to_string(),
            "R13".to_string(),
            "M=D".to_string(),
        ];
        res.append(&mut pop_code_generator::generate_pop_sp_to_r13_code());
        res
    }

    fn camel_case_filename_without_extention(filename: &str) -> String {
        let without_extention = filename.replace(".vm", "").to_lowercase();
        let mut file_name_char = without_extention.chars();
        match file_name_char.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + file_name_char.as_str(),
        }
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
        let result = push_code_generator::push_constant("7");
        assert_eq!(result, expected_result)
    }

    #[test]
    fn push_local() {
        let expected_result = vec![
            "@LCL".to_string(),
            "D=M".to_string(),
            "@1".to_string(),
            "A=D+A".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ];
        let mut code_writer = CodeWriter::new("a".to_string());
        code_writer.push("local", "1");
        assert_eq!(code_writer.generated_code, expected_result)
    }

    #[test]
    fn pop_local() {
        let expected_result = vec![
            "@LCL".to_string(),
            "D=M".to_string(),
            "@1".to_string(),
            "D=D+A".to_string(),
            "@R13".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            "@R13".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
        ];
        let mut code_writer = CodeWriter::new("a".to_string());
        code_writer.pop("local", "1");
        assert_eq!(code_writer.generated_code, expected_result)
    }

    #[test]
    fn camel_case_filename_without_extention() {
        let expected_result = "Filename";
        let result = CodeWriter::camel_case_filename_without_extention("filename.vm");
        assert_eq!(result, expected_result)
    }
    #[test]
    fn write_label() {
        let expected_result = ["null$b".to_string()];
        let mut code_writer = CodeWriter::new("a".to_string());
        code_writer.write_label("b");
        assert_eq!(code_writer.generated_code, expected_result)
    }
}

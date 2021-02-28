use std::{fs::OpenOptions, io::prelude::*};
mod arithmetic_code_generator;
mod arithmetic_command;
mod constant;
mod helper;
mod pop_code_generator;
mod push_code_generator;
mod return_address_generator;
mod segment;

pub struct CodeWriter {
    file_name: String,
    generated_code: Vec<String>,
    symbol_count: usize,
    function_name_stack: Vec<String>,
    return_address_generator: return_address_generator::ReturnAddressGenerator,
}

impl CodeWriter {
    pub fn new(file_name: String) -> CodeWriter {
        CodeWriter {
            file_name,
            generated_code: vec![],
            symbol_count: 0,
            function_name_stack: vec!["null".to_string()],
            return_address_generator: return_address_generator::ReturnAddressGenerator::new(),
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

    pub fn write_init(&mut self) {
        // TODO bootstrapコードを書く
        let mut new_code: Vec<String> = vec![];
        self.generated_code.append(&mut new_code);
    }

    pub fn push(&mut self, segment: &str, index: &str) {
        let mut new_code = push_code_generator::generate_push_code(segment, index, &self.file_name);
        self.generated_code.append(&mut new_code);
    }

    pub fn pop(&mut self, segment: &str, index: &str) {
        let mut new_code = pop_code_generator::generate_pop_code(segment, index, &self.file_name);
        self.generated_code.append(&mut new_code);
    }

    pub fn write_label(&mut self, label_name: &str) {
        let mut new_code = vec![format!(
            "({}${})",
            self.function_name_stack.last().unwrap(),
            label_name
        )];
        self.generated_code.append(&mut new_code)
    }

    pub fn write_go_to(&mut self, label_name: &str) {
        let mut new_code = vec![
            format!(
                "@{}${}",
                self.function_name_stack.last().unwrap(),
                label_name
            ),
            "0;JMP".to_string(),
        ];
        self.generated_code.append(&mut new_code)
    }

    pub fn write_if_go_to(&mut self, label_name: &str) {
        let mut new_code = vec![
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            format!(
                "@{}${}",
                self.function_name_stack.last().unwrap(),
                label_name
            ),
            "D;JNE".to_string(),
        ];

        self.generated_code.append(&mut new_code);
    }

    pub fn write_call(&mut self, function_name: &str, n_arg: &str) {
        let return_address = self.return_address_generator.generate_new_return_address();
        let mut new_code: Vec<String> = vec![];

        let mut push_code: Vec<String> = vec!["D=M".to_string()];

        push_code.append(&mut push_code_generator::generate_push_d_to_sp_code());

        // return_address,LCL,ARG,THIS,THATをstackにpush
        new_code.append(&mut vec![format!("@{}", return_address), "D=A".to_string()]);
        new_code.append(&mut push_code_generator::generate_push_d_to_sp_code());
        new_code.push("@LCL".to_string());
        new_code.append(&mut push_code.clone());
        new_code.push("@ARG".to_string());
        new_code.append(&mut push_code.clone());
        new_code.push("@THIS".to_string());
        new_code.append(&mut push_code.clone());
        new_code.push("@THAT".to_string());
        new_code.append(&mut push_code.clone());

        // ARG = SP - n_arg - 5
        new_code.append(&mut vec![
            "@SP".to_string(),
            "D=M".to_string(),
            format!("@{}", n_arg),
            "D=D-A".to_string(),
            "@5".to_string(),
            "D=D-A".to_string(),
            "@ARG".to_string(),
            "M=D".to_string(),
        ]);

        // LCL = SP
        new_code.append(&mut vec![
            "@SP".to_string(),
            "D=M".to_string(),
            "@LCL".to_string(),
            "M=D".to_string(),
        ]);

        // function_nameに制御を移す
        new_code.append(&mut vec![
            format!("@{}", function_name),
            "0;JMP".to_string(),
        ]);

        // return_addressを書いておく
        new_code.push(format!("({})", return_address));
        self.generated_code.append(&mut new_code);
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

    pub fn write_function(&mut self, function_name: &str, num_locals: &str) {
        let mut new_code: Vec<String> = vec![];
        self.function_name_stack.push(function_name.to_string());
        new_code.push(format!("({})", function_name));
        let mut push_zero_to_stack = vec!["@0".to_string(), "D=A".to_string()];
        push_zero_to_stack.append(&mut push_code_generator::generate_push_d_to_sp_code());
        for _ in 0..num_locals.parse::<i32>().unwrap() {
            new_code.append(&mut push_zero_to_stack.clone());
        }

        self.generated_code.append(&mut new_code);
    }

    pub fn write_return(&mut self) {
        let mut new_code = vec![
            "@LCL".to_string(),
            "D=M".to_string(),
            "@FRAME".to_string(),
            "M=D".to_string(),
            "@5".to_string(),
            "A=D-A".to_string(),
            "D=M".to_string(),
            "@RET".to_string(),
            "M=D".to_string(),
        ];
        new_code.append(&mut pop_code_generator::generate_pop_code(
            "argument", "0", "",
        ));
        new_code.append(&mut vec![
            "@ARG".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=D+1".to_string(),
        ]);

        new_code.append(&mut vec![
            "@FRAME".to_string(),
            "D=M".to_string(),
            "@1".to_string(),
            "A=D-A".to_string(),
            "D=M".to_string(),
            "@THAT".to_string(),
            "M=D".to_string(),
        ]);
        new_code.append(&mut vec![
            "@FRAME".to_string(),
            "D=M".to_string(),
            "@2".to_string(),
            "A=D-A".to_string(),
            "D=M".to_string(),
            "@THIS".to_string(),
            "M=D".to_string(),
        ]);
        new_code.append(&mut vec![
            "@FRAME".to_string(),
            "D=M".to_string(),
            "@3".to_string(),
            "A=D-A".to_string(),
            "D=M".to_string(),
            "@ARG".to_string(),
            "M=D".to_string(),
        ]);
        new_code.append(&mut vec![
            "@FRAME".to_string(),
            "D=M".to_string(),
            "@4".to_string(),
            "A=D-A".to_string(),
            "D=M".to_string(),
            "@LCL".to_string(),
            "M=D".to_string(),
        ]);

        new_code.append(&mut vec![
            "@RET".to_string(),
            "A=M".to_string(),
            "0;JMP".to_string(),
        ]);
        self.generated_code.append(&mut new_code);
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn write_label() {
        let expected_result = ["(null$b)".to_string()];
        let mut code_writer = CodeWriter::new("a".to_string());
        code_writer.write_label("b");
        assert_eq!(code_writer.generated_code, expected_result)
    }

    #[test]
    fn write_go_to() {
        let expected_result = ["@null$b".to_string(), "0;JMP".to_string()];
        let mut code_writer = CodeWriter::new("a".to_string());
        code_writer.write_go_to("b");
        assert_eq!(code_writer.generated_code, expected_result)
    }

    #[test]
    fn write_if_go_to() {
        let expected_result = [
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            "@null$b".to_string(),
            "D;JNE".to_string(),
        ];
        let mut code_writer = CodeWriter::new("a".to_string());
        code_writer.write_if_go_to("b");
        assert_eq!(code_writer.generated_code, expected_result)
    }

    #[test]
    fn write_function() {
        let expected_result = [
            "(Functionname)".to_string(),
            "@0".to_string(),
            "D=A".to_string(),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "@0".to_string(),
            "D=A".to_string(),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ];

        let mut code_writer = CodeWriter::new("a".to_string());
        code_writer.write_function("Functionname", "2");
        assert_eq!(code_writer.generated_code, expected_result);
        assert_eq!(code_writer.function_name_stack.len(), 2);
        assert_eq!(
            code_writer.function_name_stack.pop().unwrap(),
            "Functionname".to_string()
        )
    }
}

use std::collections::VecDeque;

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

pub struct CodeWriter {
    file_name: String,
    stack_point: usize,
    generated_code: Vec<String>,
}

impl CodeWriter {
    pub fn new(file_name: String) -> CodeWriter {
        CodeWriter {
            stack_point: 256,
            file_name,
            generated_code: vec![],
        }
    }

    pub fn push(&mut self, segment: &str, index: &str) {
        match Segment::from_str(segment) {
            Some(Segment::CONSTANT) => {
                let mut commands = CodeWriter::push_constant(self.stack_point, index);
                self.stack_point += 1;
                self.generated_code.append(&mut commands)
            }
            _ => (),
        }
    }

    fn push_constant(stack_point: usize, constant: &str) -> Vec<String> {
        vec![
            format!("@{}", constant),
            "D=A".to_string(),
            format!("@{}", stack_point),
            "M=D".to_string(),
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
            "@256".to_string(),
            "M=D".to_string(),
        ];
        let result = CodeWriter::push_constant(256, "7");
        assert_eq!(result, expected_result)
    }
}

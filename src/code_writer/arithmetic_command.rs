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

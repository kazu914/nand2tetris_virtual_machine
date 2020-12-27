pub fn neg() -> Vec<String> {
    make_one_operand_code("-")
}
pub fn not() -> Vec<String> {
    make_one_operand_code("!")
}

pub fn make_one_operand_code(operator: &str) -> Vec<String> {
    vec![
        "@SP".to_string(),
        "M=M-1".to_string(),
        "A=M".to_string(),
        format!("M={}M", operator),
        "@SP".to_string(),
        "M=M+1".to_string(),
    ]
}

pub fn add() -> Vec<String> {
    make_two_operands_code("+")
}

pub fn sub() -> Vec<String> {
    make_two_operands_code("-")
}

pub fn and() -> Vec<String> {
    make_two_operands_code("&")
}

pub fn or() -> Vec<String> {
    make_two_operands_code("|")
}

pub fn make_two_operands_code(operator: &str) -> Vec<String> {
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

pub fn eq(symbol_count: &usize) -> Vec<String> {
    make_condition_code(symbol_count, "JEQ")
}

pub fn gt(symbol_count: &usize) -> Vec<String> {
    make_condition_code(symbol_count, "JGT")
}

pub fn lt(symbol_count: &usize) -> Vec<String> {
    make_condition_code(symbol_count, "JLT")
}

pub fn make_condition_code(symbol_count: &usize, condition: &str) -> Vec<String> {
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

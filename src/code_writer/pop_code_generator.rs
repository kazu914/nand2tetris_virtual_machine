pub fn pop_segment(index: &str, segment: &str) -> Vec<String> {
    let mut res = vec![
        format!("@{}", segment),
        "D=M".to_string(),
        format!("@{}", index),
        "D=D+A".to_string(),
        "@R13".to_string(),
        "M=D".to_string(),
    ];
    res.append(&mut generate_pop_sp_to_r13_code());
    res
}

pub fn pop_pointer_and_temp(index: &str, base_address: &str) -> Vec<String> {
    let mut res = vec![
        format!("@{}", base_address),
        "D=A".to_string(),
        format!("@{}", index),
        "D=D+A".to_string(),
        "@R13".to_string(),
        "M=D".to_string(),
    ];
    res.append(&mut generate_pop_sp_to_r13_code());
    res
}

pub fn generate_pop_sp_to_r13_code() -> Vec<String> {
    vec![
        "@SP".to_string(),
        "AM=M-1".to_string(),
        "D=M".to_string(),
        "@R13".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
    ]
}

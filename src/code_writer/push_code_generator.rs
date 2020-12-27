pub fn push_constant(constant: &str) -> Vec<String> {
    let mut res = vec![format!("@{}", constant), "D=A".to_string()];
    res.append(&mut generate_push_d_to_sp_code());
    res
}

pub fn push_segment(index: &str, segment: &str) -> Vec<String> {
    let mut res = vec![
        format!("@{}", segment),
        "D=M".to_string(),
        format!("@{}", index),
        "A=D+A".to_string(),
        "D=M".to_string(),
    ];
    res.append(&mut generate_push_d_to_sp_code());
    res
}

pub fn push_static(index: &str, constant_name: &str) -> Vec<String> {
    let mut res = vec![format!("@{}.{}", constant_name, index), "D=M".to_string()];
    res.append(&mut generate_push_d_to_sp_code());
    res
}

pub fn push_pointer_and_temp(index: &str, base_address: &str) -> Vec<String> {
    let mut res = vec![
        format!("@{}", base_address),
        "D=A".to_string(),
        format!("@{}", index),
        "A=D+A".to_string(),
        "D=M".to_string(),
    ];
    res.append(&mut generate_push_d_to_sp_code());
    res
}

pub fn generate_push_d_to_sp_code() -> Vec<String> {
    vec![
        "@SP".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
        "@SP".to_string(),
        "M=M+1".to_string(),
    ]
}

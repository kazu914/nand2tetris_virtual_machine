use crate::code_writer::constant::{POINTER_BASE_ADDRESS, TEMP_BASE_ADDRESS};
use crate::code_writer::helper::camel_case_filename_without_extention;
use crate::code_writer::segment::Segment;

pub fn generate_push_code(segment: &str, index: &str, file_name: &str) -> Vec<String> {
    match Segment::from_str(segment) {
        Some(Segment::CONSTANT) => push_constant(index),
        Some(Segment::LOCAL)
        | Some(Segment::ARGUMENT)
        | Some(Segment::THIS)
        | Some(Segment::THAT) => {
            push_segment(index, Segment::to_register_alias_str(segment).as_str())
        }
        Some(Segment::POINTER) => push_pointer_and_temp(index, POINTER_BASE_ADDRESS),
        Some(Segment::TEMP) => push_pointer_and_temp(index, TEMP_BASE_ADDRESS),
        Some(Segment::STATIC) => {
            let constant_name = camel_case_filename_without_extention(file_name);
            push_static(index, &constant_name)
        }
        _ => vec![],
    }
}

fn push_constant(constant: &str) -> Vec<String> {
    let mut res = vec![format!("@{}", constant), "D=A".to_string()];
    res.append(&mut generate_push_d_to_sp_code());
    res
}

fn push_segment(index: &str, segment: &str) -> Vec<String> {
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

fn push_static(index: &str, constant_name: &str) -> Vec<String> {
    let mut res = vec![format!("@{}.{}", constant_name, index), "D=M".to_string()];
    res.append(&mut generate_push_d_to_sp_code());
    res
}

fn push_pointer_and_temp(index: &str, base_address: &str) -> Vec<String> {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_push_constant() {
        let expected_result = vec![
            "@7".to_string(),
            "D=A".to_string(),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ];
        let result = push_constant("7");
        assert_eq!(result, expected_result)
    }
}

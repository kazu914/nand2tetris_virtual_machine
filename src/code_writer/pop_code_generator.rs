use crate::code_writer::constant::{POINTER_BASE_ADDRESS, TEMP_BASE_ADDRESS};
use crate::code_writer::helper::camel_case_filename_without_extention;
use crate::code_writer::segment::Segment;

pub fn generate_pop_code(segment: &str, index: &str, file_name: &str) -> Vec<String> {
    match Segment::from_str(segment) {
        Some(Segment::LOCAL)
        | Some(Segment::ARGUMENT)
        | Some(Segment::THIS)
        | Some(Segment::THAT) => {
            pop_segment(index, Segment::to_register_alias_str(segment).as_str())
        }
        Some(Segment::POINTER) => pop_pointer_and_temp(index, POINTER_BASE_ADDRESS),
        Some(Segment::TEMP) => pop_pointer_and_temp(index, TEMP_BASE_ADDRESS),
        Some(Segment::STATIC) => {
            let constant_name = camel_case_filename_without_extention(file_name);
            pop_static(index, &constant_name)
        }
        _ => vec![],
    }
}

fn pop_segment(index: &str, segment: &str) -> Vec<String> {
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

fn pop_pointer_and_temp(index: &str, base_address: &str) -> Vec<String> {
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
fn pop_static(index: &str, constant_name: &str) -> Vec<String> {
    let mut res = vec![
        format!("@{}.{}", constant_name, index),
        "D=A".to_string(),
        "R13".to_string(),
        "M=D".to_string(),
    ];
    res.append(&mut generate_pop_sp_to_r13_code());
    res
}

fn generate_pop_sp_to_r13_code() -> Vec<String> {
    vec![
        "@SP".to_string(),
        "AM=M-1".to_string(),
        "D=M".to_string(),
        "@R13".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
    ]
}

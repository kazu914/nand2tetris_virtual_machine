pub fn camel_case_filename_without_extention(filename: &str) -> String {
    let splited_filename: Vec<&str> = filename.rsplit("/").collect();
    let without_extention = splited_filename[0].replace(".vm", "").to_lowercase();
    let mut file_name_char = without_extention.chars();
    match file_name_char.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + file_name_char.as_str(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_camel_case_filename_without_extention() {
        let expected_result = "Filename";
        let result = camel_case_filename_without_extention("../path/to/filename.vm");
        assert_eq!(result, expected_result)
    }
}

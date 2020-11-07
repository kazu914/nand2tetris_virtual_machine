use std::collections::VecDeque;

pub struct CodeWriter {
    stack: VecDeque<usize>,
    file_name: String,
}

impl CodeWriter {
    pub fn new(file_name: String) -> CodeWriter {
        CodeWriter {
            stack: VecDeque::new(),
            file_name,
        }
    }
}

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

    pub fn to_register_alias_str(segment: &str) -> String {
        match Segment::from_str(segment) {
            Some(Segment::LOCAL) => "LCL".to_string(),
            Some(Segment::ARGUMENT) => "ARG".to_string(),
            Some(Segment::THIS) => "THIS".to_string(),
            Some(Segment::THAT) => "THAT".to_string(),
            _ => panic!("{:?} has not alias name in register", segment),
        }
    }
}

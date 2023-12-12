pub struct Part {
    pub part_number: u8,
}

impl Part {
    pub fn from(part_number: u8) -> Part {
        return Part { part_number };
    }
}
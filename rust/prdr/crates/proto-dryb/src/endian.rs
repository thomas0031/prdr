#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

impl Default for Endianness {
    fn default() -> Self {
        Endianness::Little
    }
}

mod parser;
pub use parser::*;

mod input;
pub use input::*;

pub fn to_str(b: &[u8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(b) }
}

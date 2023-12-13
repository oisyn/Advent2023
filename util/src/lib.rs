mod parser;
pub use parser::*;

mod input;
pub use input::*;

mod fieldview;
pub use fieldview::*;

pub fn to_str(b: &[u8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(b) }
}

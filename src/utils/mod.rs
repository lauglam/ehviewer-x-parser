mod parser;

pub mod const_concat;
pub mod test;
pub mod input;

pub use parser::parse_u32;
pub use parser::parse_i32;
pub use parser::parse_u64;
pub use parser::parse_f32;
pub use parser::unescape;
pub use parser::trim;

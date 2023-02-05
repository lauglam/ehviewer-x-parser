mod parser;

pub mod const_concat;
pub mod test;

pub use parser::parse_usize;
pub use parser::parse_isize;
pub use parser::parse_u64;
pub use parser::parse_i64;
pub use parser::parse_f32;
pub use parser::unescape_xml;
pub use parser::trim;

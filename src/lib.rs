pub mod utils;
pub mod structures;

mod eh_url;
mod eh_config;
mod result;

pub use utils::{
    input::Input,
    const_concat::{
        transmute,
        concat,
    },
};
pub use result::{
    EhResult,
    ParseError,
    Parser,
};

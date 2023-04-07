#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(debug_assertions, allow(dead_code))]

mod app;
mod input;
mod options;

pub use app::App;
pub use input::SimpleInput;
pub use options::Options;

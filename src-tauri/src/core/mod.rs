use std::fmt::Display;

mod blueprints_consts;
mod blueprints_funcs;
mod blueprints_lib;

pub use blueprints_funcs::{
    generate_mini_static_image_blueprint, 
    generate_mini_dynamic_image_blueprint,
    generate_screen_blueprint,
    get_gif_duration,
};

#[derive(Debug)]
pub struct BluePrintError(String);

impl Display for BluePrintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl std::error::Error for BluePrintError {}

impl From<anyhow::Error> for BluePrintError {
    fn from(value: anyhow::Error) -> Self {
        Self(value.to_string())
    }
}
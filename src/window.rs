extern crate gfx_hal as hal;

#[cfg(feature = "gl")]
#[path=""]
mod gl_crates {
    extern crate gfx_backend_gl as back;
    extern crate glutin as win;
}

#[cfg(feature = "gl")]
pub use gl_crates::*;

use conf;
use error::EngineResult;

pub struct Window {}
pub struct WindowBuilder {}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn with_settings(&mut self, _conf: &conf::Conf) -> &Self {

        self
    }

    pub fn build(&self) -> EngineResult<Window> {
        Ok(Window {

        })
    }
}

impl Window {
    
}
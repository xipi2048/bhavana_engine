use conf;
use error::EngineResult;

pub struct Context {
    
}

pub struct ContextBuilder {
    conf: conf::Conf
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            conf: conf::Conf::new()
        }
    }

    pub fn window_settings(&self, _settings: conf::WindowSettings) -> &Self {

        self
    }

    pub fn build(&self) -> EngineResult<Context> {
        Ok(Context {
        })
    }
}
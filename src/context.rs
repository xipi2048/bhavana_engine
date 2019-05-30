use conf;

pub struct Context {}
pub struct ContextBuilder {
    _conf: conf::Conf
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            _conf: conf::Conf::new()
        }
    }

    pub fn window_settings(self, _settings: conf::WindowSettings) -> Self {

        self
    }

    pub fn build(self) -> Context {
        Context {

        }
    }
}

impl Context {
    
}
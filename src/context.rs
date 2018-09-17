use conf;

pub struct Context {}
impl Context {
    conf: conf::Conf
}

pub struct ContextBuilder {}
impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            conf: conf::Conf::new()
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
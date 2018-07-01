use conf;

pub struct Context {}
impl Context {

}

pub struct ContextBuilder {}
impl ContextBuilder {
    pub fn new() -> Self {
        Self {

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
use conf;
use error::EngineResult;

pub struct System {}

pub struct SystemBuilder {
    conf: conf::Conf,
}

impl SystemBuilder {
    pub fn new() -> Self {
        Self {
            conf: conf::Conf::new(),
        }
    }

    pub fn window_settings(&self, _settings: conf::WindowSettings) -> &Self {
        self
    }

    pub fn build(&self) -> EngineResult<System> {
        Ok(System {})
    }

    pub fn run(&self) -> EngineResult<()> {
        /*
            
        */
        
        Ok(())
    }
}

use conf;
use window;
use error::EngineResult;

pub struct System {
    _window: window::Window
}

pub struct SystemBuilder {
    conf: conf::Conf,
}

impl SystemBuilder {
    pub fn new() -> Self {
        Self {
            conf: conf::Conf::new(),
        }
    }

    pub fn window_settings(&mut self, settings: conf::WindowSettings) -> &Self {
        self.conf.window_settings = settings;
        self
    }

    pub fn build(&self) -> EngineResult<System> {
        let window = window::WindowBuilder::new()
            .with_settings(&self.conf)
            .build()
            .unwrap();

        Ok(System {
          _window: window  
        })
    }    
}

impl System {
    pub fn run(&self) -> EngineResult<()> {
        /*
            
        */
        
        Ok(())
    }
}

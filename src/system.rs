use conf;
use window;
use error::EngineResult;

pub struct System {
    window: window::Window
}

pub struct SystemBuilder {
    conf: conf::Conf,
}

impl SystemBuilder {
    pub fn new() -> Self {
        Self {
            conf: conf::Conf::default(),
        }
    }

    pub fn window_settings(&mut self, settings: conf::WindowSettings) -> &Self {
        self.conf.window_settings = settings;
        self
    }

    pub fn with_state_manager(&self, _state_manager: state::StateManager) -> &self {
        self
    }

    pub fn build(&self) -> EngineResult<System> {
        let window = window::WindowBuilder::new()
            .with_settings(&self.conf)
            .build()
            .unwrap();

        Ok(System {
          window: window  
        })
    }    
}

impl System {
    pub fn run_forever<T>(&mut self, callback: T) 
        where T: FnMut(winit::Event) -> winit::ControlFlow
    {        
        self.window.run_forever(callback)
    }
}

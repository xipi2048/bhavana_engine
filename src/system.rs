use crate::conf;
use crate::window;
use crate::state;
use crate::error::EngineResult;

pub struct System {
    window: window::Window
}

pub struct SystemBuilder {
    conf: conf::Conf,
}

impl SystemBuilder {
    pub fn new() -> Self {
        Self {
            conf: conf::Conf::default()
        }
    }

    pub fn window_settings(&mut self, settings: conf::WindowSettings) -> &mut Self {
        self.conf.window_settings = settings;
        self
    }

    pub fn with_state_manager<T>(&mut self, _state_manager: T) -> &mut Self 
    where 
        T: state::StateManager
    {
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
    // pub fn run<T>(&mut self, callback: T) 
    //     where T: FnMut(winit::Event) -> winit::ControlFlow
    // {        
    //     self.window.run(callback)
    // }

    pub fn run<T>(&mut self, state_manager: T)
        where T: state::StateManager
    {
        self.window.run(|event| {
            state_manager.handle_event(event)
        });
    }
}

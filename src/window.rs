extern crate gfx_hal as hal;

extern crate gfx_backend_gl as back;
extern crate glutin as win;

use conf;
use error::EngineResult;

pub struct Window {
    _window: win::Window,
    events_loop: win::EventsLoop
}
pub struct WindowBuilder {
    conf: conf::Conf
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            conf: conf::Conf::default()
        }
    }

    pub fn with_settings(&mut self, conf: &conf::Conf) -> &Self {
        // TODO: remove clone
        self.conf = conf.clone();
        self
    }

    pub fn build(&self) -> EngineResult<Window> {
        let win_set = &self.conf.window_settings;
        let events_loop = win::EventsLoop::new();
        let window = win::WindowBuilder::new()
            .with_title(win_set.title)
            .with_dimensions((win_set.width, win_set.height).into())
            .build(&events_loop)
            .unwrap();

        Ok(Window{
            _window: window,
            events_loop: events_loop        
        })
    }
}

impl Window {
    pub fn run_forever<T>(&mut self, callback: T) 
        where T: FnMut(win::Event) -> win::ControlFlow
    {        
        self.events_loop.run_forever(callback)
    }
}
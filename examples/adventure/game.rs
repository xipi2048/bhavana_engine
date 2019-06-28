use bhavana_engine::{
    winit::{Event, WindowEvent, ControlFlow},
    state::StateManager
};

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl StateManager for Game {
    fn handle_event(&self, event: Event) -> ControlFlow {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => ControlFlow::Break,
            _ => ControlFlow::Continue
        }
    }
}
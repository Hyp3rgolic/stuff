use winit::{
    event::*,
};
pub struct Input {
    pub space: bool,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

impl Input {
    pub fn init() -> Self {
        Self { space: false, w: false,
        a: false, s: false, d: false}
    }

    pub fn handle(&mut self, event: &WindowEvent) -> bool {
        match event {
            //1
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                match keycode {
                    VirtualKeyCode::Space => {  self.space = *state == ElementState::Pressed; true}
                    VirtualKeyCode::W => {  self.w = *state == ElementState::Pressed; true}
                    VirtualKeyCode::A => {  self.a = *state == ElementState::Pressed; true}
                    VirtualKeyCode::S => {  self.s = *state == ElementState::Pressed; true}
                    VirtualKeyCode::D => {  self.d = *state == ElementState::Pressed; true}

                    _ => false
                }

            }
            //
            _ => false,
        }
    }
}

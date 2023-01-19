use winit::event::{ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent};

use crate::math::geo::V2;

#[derive(Clone)]
pub enum KeyAction {
    Pressed(VirtualKeyCode),
    Released(VirtualKeyCode),
}

#[derive(Clone)]
pub enum MouseAction {
    Pressed(usize),
    Released(usize),
}

#[derive(Clone)]
pub enum TextChar {
    Char(char),
    Back,
}

#[derive(Clone)]
pub struct Input {
    pub mouse_actions: Vec<MouseAction>,
    pub key_actions: Vec<KeyAction>,
    pub key_held: [bool; 255],
    pub mouse_held: [bool; 255],
    pub mouse_position: Option<V2>,
    pub mouse_position_prev: Option<V2>,
    pub scroll_diff: f32,
    pub text: Vec<TextChar>,
}

impl Input {
    pub fn new () -> Self {
        Self {
            mouse_actions: Vec::new(),
            key_actions: Vec::new(),
            key_held: [false; 255],
            mouse_held: [false; 255],
            mouse_position: None,
            mouse_position_prev: None,
            scroll_diff: 0.0,
            text: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.mouse_actions.clear();
        self.key_actions.clear();
        self.scroll_diff = 0.0;
        self.mouse_position_prev = self.mouse_position;
        self.text.clear();
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, ..} => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            if !self.key_held[keycode as usize] {
                                self.key_actions.push(KeyAction::Pressed(keycode));
                            }
                            self.key_held[keycode as usize] = true;
                            if let VirtualKeyCode::Back = keycode {
                                self.text.push(TextChar::Back);
                            }
                        }

                        ElementState::Released => {
                            self.key_held[keycode as usize] = false;
                            self.key_actions.push(KeyAction::Released(keycode));
                        }
                    }
                }
            }

            WindowEvent::ReceivedCharacter(c) => {
                let c = *c;
                if c != '\x08' && c != '\r' && c != '\n' {
                    self.text.push(TextChar::Char(c));
                }
            }

            WindowEvent::CursorMoved { position, ..} => {
                self.mouse_position = Some(V2::new(position.x as f32, position.y as f32));
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                let button = mouse_button_to_int(button);
                self.mouse_held[button] = true;
                self.mouse_actions.push(MouseAction::Pressed(button));
            }

            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                let button = mouse_button_to_int(button);
                self.mouse_held[button] = false;
                self.mouse_actions.push(MouseAction::Released(button));
            }

            WindowEvent::MouseWheel { delta, ..} => {
                const PIXELS_PER_LINE: f64 = 38.0;

                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        self.scroll_diff += y;
                    }
                    MouseScrollDelta::PixelDelta(delta) => {
                        self.scroll_diff += (delta.y / PIXELS_PER_LINE) as f32;
                    }
                }
            }

            _ => {}
        }
    }
}

pub fn mouse_button_to_int(button: &MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(byte) => *byte as usize,
    }
}

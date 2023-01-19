use std::path::PathBuf;

use winit::dpi::PhysicalSize;
use winit::event::{Event, MouseButton, VirtualKeyCode, WindowEvent};

use crate::math::geo::V2;
use crate::sys::input::{Input, KeyAction, mouse_button_to_int, MouseAction, TextChar};

#[derive(Clone)]
pub struct InputSubsystem {
    current: Option<Input>,
    dropped_file: Option<PathBuf>,
    window_resized: Option<PhysicalSize<u32>>,
    window_size: Option<(u32, u32)>,
    scale_factor_changed: Option<f64>,
    scale_factor: Option<f64>,
    destroyed: bool,
    close_requested: bool,
}

impl Default for InputSubsystem {
    fn default() -> Self {
        return Self::new()
    }
}

impl InputSubsystem {
    pub fn new () -> Self {
        Self {
            current: Some(Input::new()),
            dropped_file: None,
            window_resized: None,
            window_size: None,
            scale_factor_changed: None,
            scale_factor: None,
            destroyed: false,
            close_requested: false,
        }
    }

    pub fn update<T>(&mut self, event: &Event<T>) -> bool {
        match &event {
            Event::NewEvents(_) => {
                self.flush();
                false
            }
            Event::WindowEvent {event, ..} => {
                self.process_window_event(event);
                false
            }
            Event::MainEventsCleared => true,
            _ => false,
        }
    }

    pub fn flush_with_window_events(&mut self, events: &[WindowEvent]) {
        self.flush();
        for event in events {
            self.process_window_event(event);
        }
    }

    pub fn flush(&mut self) {
        self.dropped_file = None;
        self.window_resized = None;
        self.scale_factor_changed = None;
        self.close_requested = false;
        if let Some(current) = &mut self.current {
            current.update();
        }
    }

    fn process_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.close_requested = true,
            WindowEvent::Destroyed => self.destroyed = true,
            WindowEvent::Focused(false) => self.current = None,
            WindowEvent::Focused(true) => {
                if self.current.is_none() {
                    self.current = Some(Input::new());
                }
            }
            WindowEvent::DroppedFile(path) => self.dropped_file = Some(path.clone()),
            WindowEvent::Resized(size) => {
                self.window_resized = Some(*size);
                self.window_size = Some((*size).into());
            }
            WindowEvent::ScaleFactorChanged { scale_factor, ..} => {
                self.scale_factor_changed = Some(*scale_factor);
                self.scale_factor = Some(*scale_factor);
            }
            _ => {}
        }

        if let Some(current) = &mut self.current {
            current.handle_event(event);
        }
    }

    pub fn key_pressed(&self, check_key_code: VirtualKeyCode) -> bool {
        if let Some(current) = &self.current {
            for action in &current.key_actions {
                if let KeyAction::Pressed(key_code) = *action {
                    if key_code == check_key_code {
                         return true;
                    }
                }
            }
        }
        false
    }

    pub fn key_released(&self, check_key_code: VirtualKeyCode) -> bool {
        if let Some(current) = &self.current {
            for action in &current.key_actions {
                if let KeyAction::Released(key_code) = *action {
                    if key_code == check_key_code {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn key_held(&self, key_code: VirtualKeyCode) -> bool {
        match &self.current {
            Some(current) => current.key_held[key_code as usize],
            None => false,
        }
    }

    pub fn held_shift(&self) -> bool {
        self.key_held(VirtualKeyCode::LShift) || self.key_held(VirtualKeyCode::RShift)
    }

    pub fn held_alt(&self) -> bool {
        self.key_held(VirtualKeyCode::LAlt) || self.key_held(VirtualKeyCode::RAlt)
    }

    pub fn held_control(&self) -> bool {
        self.key_held(VirtualKeyCode::LControl) || self.key_held(VirtualKeyCode::RControl)
    }

    // left => 0, right => 1, middle => 2
    pub fn mouse_pressed(&self, check_mouse_button: MouseButton) -> bool {
        if let Some(current) = &self.current {
            for action in &current.mouse_actions {
                if let MouseAction::Pressed(key_code) = *action {
                    if key_code == mouse_button_to_int(&check_mouse_button) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn mouse_released(&self, check_mouse_button: MouseButton) -> bool {
        if let Some(current) = &self.current {
            for action in &current.mouse_actions {
                if let MouseAction::Released(key_code) = *action {
                    if key_code == mouse_button_to_int(&check_mouse_button) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn mouse_held(&self, mouse_button: MouseButton) -> bool {
        let mbidx = mouse_button_to_int(&mouse_button);
        match &self.current {
            Some(current) => current.mouse_held[mbidx as usize],
            None => false,
        }
    }

    pub fn scroll_diff(&self) -> f32 {
        match &self.current {
            Some(current) => current.scroll_diff,
            None => 0.0,
        }
    }

    pub fn mouse_pos(&self) -> Option<V2> {
        match &self.current {
            Some(current) => current.mouse_position,
            None => None,
        }
    }

    pub fn mouse_diff(&self) -> V2 {
        if let Some(current_input) = &self.current {
            if let Some(curr_pos) = current_input.mouse_position {
                if let Some(prev_pos) = current_input.mouse_position_prev {
                    return curr_pos - prev_pos;
                }
            }
        }
        V2::new(0.0, 0.0)
    }

    pub fn text(&self) -> Vec<TextChar> {
        match &self.current {
            Some(current) => current.text.clone(),
            None => vec![],
        }
    }

    pub fn dropped_file(&self) -> Option<PathBuf> {
        self.dropped_file.clone()
    }

    pub fn window_resized(&self) -> Option<PhysicalSize<u32>> {
        self.window_resized
    }

    pub fn resolution(&self) -> Option<(u32, u32)> {
        self.window_size
    }

    pub fn scale_factor_changed(&self) -> Option<f64> {
        self.scale_factor_changed
    }

    pub fn scale_factor(&self) -> Option<f64> {
        self.scale_factor
    }

    pub fn destroyed(&self) -> bool {
        self.destroyed
    }

    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}


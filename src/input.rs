use crate::math::Vec2;
use crate::vec2;
use sdl2::EventPump;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::mouse::MouseButton;

use std::collections::HashSet;
pub struct Input {
    event_pump: EventPump,
    quit: bool,
    prev_keys: HashSet<Keycode>,
    new_keys: HashSet<Keycode>,
    old_keys: HashSet<Keycode>,
    prev_mouse: HashSet<MouseButton>,
    new_mouse: HashSet<MouseButton>,
    old_mouse: HashSet<MouseButton>,
    mouse_position: Vec2,
}
impl Input {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
            quit: false,
            prev_keys: HashSet::new(),
            new_keys: HashSet::new(),
            old_keys: HashSet::new(),
            prev_mouse: HashSet::new(),
            new_mouse: HashSet::new(),
            old_mouse: HashSet::new(),
            mouse_position: Vec2::new(0.0, 0.0),
        }
    }
    pub fn quit(&self) -> bool {
        self.quit
    }
    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.prev_keys.contains(&key)
    }
    pub fn is_key_clicked(&self, key: Keycode) -> bool {
        self.new_keys.contains(&key)
    }
    pub fn is_key_released(&self, key: Keycode) -> bool {
        self.old_keys.contains(&key)
    }
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
    pub fn is_mouse_pressed(&self, btn: MouseButton) -> bool {
        self.prev_mouse.contains(&btn)
    }
    pub fn is_mouse_clicked(&self, btn: MouseButton) -> bool {
        self.new_mouse.contains(&btn)
    }
    pub fn is_mouse_released(&self, btn: MouseButton) -> bool {
        self.old_mouse.contains(&btn)
    }
    pub fn update(&mut self) {
        let keys = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        let mouse: HashSet<MouseButton> = self
            .event_pump
            .mouse_state()
            .pressed_mouse_buttons()
            .collect();
        self.new_mouse = &mouse - &self.prev_mouse;
        self.old_mouse = &self.prev_mouse - &mouse;

        self.new_keys = &keys - &self.prev_keys;
        self.old_keys = &self.prev_keys - &keys;

        self.prev_mouse = mouse;
        self.prev_keys = keys;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit = true,
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_position = vec2!(x, y);
                }
                _ => {}
            }
        }
    }
}

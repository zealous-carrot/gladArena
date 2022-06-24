use crate::Input;

use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Layout {
    frame: Rect,
    frame_color: Color,
    bg_color: Color,
    switching: bool,
    state: u8,
    tabs: Vec<Rect>,
}
impl Layout {
    pub fn new(frame_w: u32, frame_h: u32) -> Self {
        Self {
            frame: Rect::new(0, 0, frame_w, frame_h),
            bg_color: Color::BLACK,
            switching: false,
            state: 0,
            frame_color: Color::RGB(226, 228, 223),
            tabs: vec![Rect::new(0, 0, 97, 59)],
        }
    }
    pub fn bg_color(&self) -> Color {
        self.bg_color
    }
    pub fn frame(&self) -> Rect {
        self.frame
    }
    pub fn frame_color(&self) -> Color {
        self.frame_color
    }
    pub fn state(&self) -> u8 {
        self.state
    }
    pub fn switching(&self) -> bool {
        self.switching
    }
    pub fn render(&self, display: &mut WindowCanvas) {
        display
            .set_logical_size(self.frame.w as u32, self.frame.h as u32)
            .unwrap();

        display.set_draw_color(self.frame_color);

        for tab in &self.tabs {
            display.draw_rect(*tab).unwrap();
        }
    }
    pub fn switch(&mut self, switch: bool, state: u8) {
        if switch {
            self.state = state;
        }
    }
}

pub fn text(
    display: &mut WindowCanvas,
    text: &str,
    text_color: Color,
    x: i32,
    y: i32,
    factor: f32,
) {
    let ttf_ctx = sdl2::ttf::init().unwrap();

    let font = ttf_ctx.load_font("res/font.ttf", 18).unwrap();

    let mut dim = font.size_of(text).unwrap();
    dim.0 = (dim.0 as f32 * factor) as u32;
    dim.1 = (dim.1 as f32 * factor) as u32;
    let rect = Rect::new(x - (dim.0 / 2) as i32, y - (dim.1 / 2) as i32, dim.0, dim.1);
    let surf = font.render(text).blended(text_color).unwrap();
    let texture_creator = display.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surf).unwrap();
    display.copy(&texture, None, rect).unwrap();
}

pub fn text_ex(display: &mut WindowCanvas, texts: Vec<(&str, Color)>, x: i32, y: i32, factor: f32) {
    let ttf_ctx = sdl2::ttf::init().unwrap();
    let font = ttf_ctx.load_font("res/font.ttf", 18).unwrap();
    let mut tx = x;
    let mut ty = y;
    for (text, color) in &texts {
        let mut dim = font.size_of(text).unwrap();
        dim.0 = (dim.0 as f32 * factor) as u32;
        dim.1 = (dim.1 as f32 * factor) as u32;
        let rect = Rect::new(tx, ty, dim.0, dim.1);
        let surf = font.render(text).blended(*color).unwrap();
        let texture_creator = display.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surf).unwrap();
        display.copy(&texture, None, rect).unwrap();
        tx += dim.0 as i32;
    }
}

pub fn button(
    display: &mut WindowCanvas,
    text: &str,
    mut text_color: Color,
    x: i32,
    y: i32,
    input: &Input,
    factor: f32,
) -> bool {
    let ttf_ctx = sdl2::ttf::init().unwrap();

    let font = ttf_ctx.load_font("res/font.ttf", 18).unwrap();
    let mut dim = font.size_of(text).unwrap();
    dim.0 = (dim.0 as f32 * factor) as u32;
    dim.1 = (dim.1 as f32 * factor) as u32;
    let rect = Rect::new(x - (dim.0 / 2) as i32, y - (dim.1 / 2) as i32, dim.0, dim.1);

    let surf = font.render(text).blended(text_color).unwrap();
    let texture_creator = display.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surf).unwrap();
    display.copy(&texture, None, rect).unwrap();

    if input.mouse_position().x > rect.x as f32
        && input.mouse_position().x < (rect.x + rect.w) as f32
        && input.mouse_position().y > (rect.y) as f32
        && input.mouse_position().y < (rect.y + rect.h) as f32
    {
        if input.is_mouse_released(MouseButton::Left) {
            return true;
        } else {
            false
        }
    } else {
        false
    }
}

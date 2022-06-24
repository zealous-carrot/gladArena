use crate::point;
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Point,
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};
use std::collections::HashMap;

pub fn load_textures<'a>(
    texture_creator: &'a mut TextureCreator<WindowContext>,
    path: String,
) -> HashMap<String, Texture<'a>> {
    let mut cache = HashMap::new();
    let mut name;
    let mut trimmed_name: String;
    for file in std::fs::read_dir(path).unwrap() {
        let nfile = file.unwrap();
        name = nfile.file_name();
        trimmed_name = name.into_string().unwrap().replace(".png", "");
        println!("{:?}", trimmed_name);
        println!("{:?}", nfile.path().to_str().unwrap());
        let texture = texture_creator
            .load_texture(nfile.path().to_str().unwrap())
            .unwrap();
        cache.insert(trimmed_name, texture); //never ever unwrap() while pushing a new key,else it will return None
    }
    return cache;
}

pub fn draw_circle(display: &mut WindowCanvas, center: (i32, i32), r: i32, color: Color) {
    let mut x = 0;
    let mut y = r;
    let mut d = 3 - 2 * r;
    display.set_draw_color(color);
    display.draw_point(point!(center.0 + x, center.1 + y));
    display.draw_point(point!(center.0 - x, center.1 + y));
    display.draw_point(point!(center.0 + x, center.1 - y));
    display.draw_point(point!(center.0 - x, center.1 - y));
    display.draw_point(point!(center.0 + y, center.1 + x));
    display.draw_point(point!(center.0 - y, center.1 + x));
    display.draw_point(point!(center.0 + y, center.1 - x));
    display.draw_point(point!(center.0 - y, center.1 - x));
    while y >= x {
        x += 1;
        if d > 0 {
            y -= 1;
            d = d + 4 * (x - y) + 10;
        } else {
            d = d + 4 * x + 6;
        }

        display.draw_point(point!(center.0 + x, center.1 + y));
        display.draw_point(point!(center.0 - x, center.1 + y));
        display.draw_point(point!(center.0 + x, center.1 - y));
        display.draw_point(point!(center.0 - x, center.1 - y));
        display.draw_point(point!(center.0 + y, center.1 + x));
        display.draw_point(point!(center.0 - y, center.1 + x));
        display.draw_point(point!(center.0 + y, center.1 - x));
        display.draw_point(point!(center.0 - y, center.1 - x));
    }
}

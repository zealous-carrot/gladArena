use crate::components::*;
use crate::entities::Wanderer;
use crate::input::Input;
use crate::math::*;
use crate::player::Player;
use crate::{point, rect};
use rand::prelude::*;
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{BlendMode, Texture, WindowCanvas},
};
use std::collections::HashMap;
pub struct Ecosystem {
    bubbles: Vec<Particle>,
    decor: Vec<Decor>,
    timer: f32,
    tiles: Vec<Rect>,
    pub players: Vec<Player>,
    messages: Vec<(f32, String)>,
    bugs: [Wanderer; 2],
}

impl Ecosystem {
    pub fn new() -> Self {
        let tiles = vec![
            Rect::new(0, 42, 24, 17),
            Rect::new(60, 42, 37, 17),
            Rect::new(0, 13, 1, 30),
            Rect::new(96, 13, 1, 30),
            Rect::new(0, 0, 97, 14),
            Rect::new(35, 42, 12, 4),
            Rect::new(23, 58, 38, 1),
        ];
        let mut bugs = [
            Wanderer::new(thread_rng().gen_range(0..97), 40, "caterpillar"),
            Wanderer::new(thread_rng().gen_range(0..97), 40, "caterpillar"),
        ];
        Self {
            bubbles: Vec::new(),
            decor: vec![
                Decor::new((7, 46), "sewage".to_string()),
                Decor::new((75, 46), "sewage".to_string()),
                Decor::new((34, 22), "fountain".to_string()),
                Decor::new((52, 14), "vine".to_string()),
            ],
            timer: 1.6,
            tiles,
            players: vec![Player::new(14, 22, "p1"), Player::new(97 - 14, 22, "p2")],
            bugs, // vines: Vec::new(),
            messages: Vec::new(),
        }
    }
    pub fn update(&mut self, dt: f32, input: &Input, camera: &mut Vec2) {
        self.timer += dt;
        if self.timer > 1.6 {
            self.timer = 0.0;
            let iterations: usize = thread_rng().gen_range(4..8);
            for _i in 0..iterations {
                self.bubbles
                    .push(Particle::new(thread_rng().gen_range(24f32..59f32), 57f32));
            }
        }
        for decor in &mut self.decor {
            decor.animator.animate(dt);
        }
        self.bubbles.retain_mut(|bubble| {
            bubble.y -= dt * bubble.acceleration.y;
            bubble.x += dt * thread_rng().gen_range(-bubble.acceleration.x..bubble.acceleration.x);

            bubble.alpha -= dt * 120.0;
            bubble.alpha > 0.0
        });
        let bullets0 = self.players[0].gun.bullets.clone();
        for b0 in bullets0 {
            if self.players[1]
                .entity
                .rect
                .contains_point(point!(b0.x, b0.y))
            {
                camera.x += thread_rng().gen_range(-4.0..4.0);
                camera.y += thread_rng().gen_range(-4.0..4.0);

                self.players[1].entity.rect.x += (10.0
                    * self.players[1].entity.physics.acceleration.x
                    * self.players[0].entity.physics.dir
                    * -1.0) as i32;
                self.players[1].entity.hp.0 -= 1.0;
            }
        }
        let bullets1 = self.players[1].gun.bullets.clone();
        for b1 in bullets1 {
            if self.players[0]
                .entity
                .rect
                .contains_point(point!(b1.x, b1.y))
            {
                camera.x += thread_rng().gen_range(-4.0..4.0);
                camera.y += thread_rng().gen_range(-4.0..4.0);
                self.players[0].entity.rect.x += (10f32
                    * self.players[0].entity.physics.acceleration.x
                    * self.players[1].entity.physics.dir
                    * -1.0) as i32;
                {
                    self.players[0].entity.hp.0 -= 1.0;
                }
            }
        }
        println!("{:?}", self.players[0].is_dead());
        self.players[0].update(
            dt,
            [
                input.is_key_pressed(Keycode::D),
                input.is_key_pressed(Keycode::A),
                input.is_key_clicked(Keycode::W),
                input.is_key_pressed(Keycode::S),
            ],
            &self.tiles,
        );
        self.players[1].update(
            dt,
            [
                input.is_key_pressed(Keycode::Right),
                input.is_key_pressed(Keycode::Left),
                input.is_key_clicked(Keycode::Up),
                input.is_key_pressed(Keycode::Down),
            ],
            &self.tiles,
        );

        for bug in &mut self.bugs {
            bug.update(dt, &self.tiles);
        }
    }
    pub fn render<'a>(
        &self,
        display: &mut WindowCanvas,
        cache: &mut HashMap<String, Texture<'a>>,
        camera: Vec2,
    ) {
        let x_offset = camera.x as i32;
        let y_offset = camera.y as i32;
        display.set_blend_mode(BlendMode::Blend);

        for bubble in &self.bubbles {
            display.set_draw_color(Color::RGBA(146, 146, 156, bubble.alpha as u8));
            display.draw_point(point!(bubble.x - camera.x, bubble.y - camera.y));
        }
        display.copy(
            cache.get_mut("fish_skeleton").unwrap(),
            None,
            Rect::new(24 - x_offset, 55 - y_offset, 6, 3),
        );
        display.copy(
            cache.get_mut("sea_grass1").unwrap(),
            None,
            Rect::new(37 - x_offset, 54 - y_offset, 6, 4),
        );
        display.copy(
            cache.get_mut("sea_grass2").unwrap(),
            None,
            Rect::new(53 - x_offset, 54 - y_offset, 6, 4),
        );
        display.set_draw_color(Color::RGBA(146, 146, 156, 79));
        display.fill_rect(Rect::new(24 - x_offset, 42 - y_offset, 36, 16));
        display.set_blend_mode(BlendMode::None);

        for decor in &self.decor {
            decor.render(display, cache, (x_offset, y_offset));
        }
        display
            .copy(
                cache.get_mut("pillar").unwrap(),
                None,
                Rect::new(88 - x_offset, 21 - y_offset, 5, 20),
            )
            .unwrap();

        for bug in &self.bugs {
            bug.render(display, cache, (x_offset, y_offset));
        }

        for player in &self.players {
            player.render(display, cache, (x_offset, y_offset));
        }
        display
            .copy(
                cache.get_mut("chain").unwrap(),
                None,
                Rect::new(18 - x_offset, 14 - y_offset, 3, 13),
            )
            .unwrap();
        display
            .copy(
                cache.get_mut("chain").unwrap(),
                None,
                Rect::new(79 - x_offset, 14 - y_offset, 3, 13),
            )
            .unwrap();
        display.set_draw_color(Color::RGB(168, 181, 174));

        for tile in &self.tiles {
            display.draw_rect(rect!(tile.x - x_offset, tile.y - y_offset, tile.w, tile.h));
        }
        display.copy(
            cache.get_mut("grass1").unwrap(),
            None,
            Rect::new(11 - x_offset, 13 - y_offset, 4, 4),
        );
        display.copy(
            cache.get_mut("grass2").unwrap(),
            None,
            Rect::new(37 - x_offset, 12 - y_offset, 4, 4),
        );
        display.copy(
            cache.get_mut("grass3").unwrap(),
            None,
            Rect::new(0 - x_offset, 26 - y_offset, 4, 4),
        );
        display.copy(
            cache.get_mut("grass4").unwrap(),
            None,
            Rect::new(93 - x_offset, 40 - y_offset, 4, 4),
        );
        display.copy(
            cache.get_mut("grass5").unwrap(),
            None,
            Rect::new(78 - x_offset, 40 - y_offset, 4, 4),
        );
        display.copy(
            cache.get_mut("grass6").unwrap(),
            None,
            Rect::new(6 - x_offset, 40 - y_offset, 4, 4),
        );
    }
}

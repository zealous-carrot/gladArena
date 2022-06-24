use crate::entities::{Entity, Gun};

use crate::{
    components::{Animator, Particle},
    math::Vec2,
    point, rect,
    ui::text,
};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::BlendMode,
    render::Texture,
    render::WindowCanvas,
};

use rand::prelude::*;

use std::collections::HashMap;

pub struct Player {
    pub entity: Entity,
    pub gun: Gun,
    dash_timer: f32,
    trail_timer: f32,
    dashing: bool,
    grounded: bool,
    shooting: bool,
    trail: Vec<Particle>,
    pub title: String,
}
impl Player {
    pub fn new(x: i32, y: i32, title: &str) -> Self {
        let mut rect = Rect::new(x, y, 3, 4);

        Self {
            entity: Entity::new(
                rect,
                Vec2::new(8.0, 9.8), //vy = mass * grav
                8.0,
                Animator::new(rect!(0, 0, rect.w, rect.h), 3, 0.1),
            ),
            gun: Gun::new(1.0, 0.3),
            dash_timer: 0.0,
            trail_timer: 0.0,
            dashing: false,
            grounded: false,
            shooting: false,
            trail: Vec::new(),
            title: title.to_string(),
        }
    }
    pub fn pos(&self) -> Vec2 {
        self.entity.physics.pos
    }
    pub fn is_dead(&self) -> bool {
        self.entity.hp.0 <= 0.0
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn update(&mut self, dt: f32, controller: [bool; 4], tiles: &Vec<Rect>) {
        let mut max_vel = Vec2::new(24.0, 120.0);
        let friction = 0.9;
        let mut air_resistance = 0.99;
        let in_water = self.entity.rect.has_intersection(Rect::new(24, 42, 36, 16));
        if in_water {
            air_resistance = 0.8;
        }
        let gravity = 4.0;
        self.entity.physics.acceleration = Vec2::new(0.0, 0.0);
        if controller[0] && !controller[1] {
            self.entity.physics.dir = 1.0;
            self.entity.physics.acceleration.x = 4.0;
        }

        if controller[1] && !controller[0] {
            self.entity.physics.dir = -1.0;
            self.entity.physics.acceleration.x = 4.0;
        }
        if controller[2] && (self.grounded || in_water) {
            self.entity.physics.acceleration.y = -max_vel.y;
        }
        // if !self.grounded {
        self.entity.physics.vel.y *= air_resistance;
        // }
        if controller[3] {
            self.shooting = true;
        }
        if self.shooting {
            self.gun.cool_down_timer += dt;
            if self.gun.cool_down_timer > self.gun.per_bullet_time {
                self.shooting = false;
                self.gun.bullets.push({
                    let mut p = Particle::new(
                        (self.entity.rect.x + 4 * self.entity.physics.dir as i32) as f32,
                        (self.entity.rect.center().y - 1) as f32,
                    );
                    p.acceleration.x *= self.entity.physics.dir;
                    p
                });
                self.gun.cool_down_timer = 0.0;
            }
        }
        // if controller[3] {
        //     self.dashing = true;
        //     for i in 0..6 {
        //         self.trail.push(Particle::new(
        //             (self.entity.rect.x + self.entity.rect.w / 2) as f32,
        //             (self.entity.rect.y + self.entity.rect.h / 2) as f32,
        //         ));
        //     }
        // }
        if self.dashing {
            self.dash_timer += dt;
            self.entity.physics.acceleration.x = 48.0;
            max_vel.x = 160.0;

            if self.dash_timer > 0.08 {
                self.dash_timer = 0.0;
                self.dashing = false;
            }
            // if self.entity.rect.has_intersection(entity.rect) {
            //     entity.hp.0 -= self.entity.;
            // }
        }
        self.entity.physics.vel.x += self.entity.physics.dir * self.entity.physics.acceleration.x;
        self.entity.physics.vel.x *= friction;
        self.entity.physics.vel.x = self.entity.physics.vel.x as i32 as f32;
        if self.entity.physics.vel.x > max_vel.x {
            self.entity.physics.vel.x = max_vel.x;
        }
        if self.entity.physics.vel.x < -max_vel.x {
            self.entity.physics.vel.x = -max_vel.x;
        }
        self.entity.physics.pos.x += self.entity.physics.vel.x * dt;
        self.entity.rect.x = self.entity.physics.pos.x as i32;
        for tile in tiles {
            if self.entity.rect.has_intersection(*tile) {
                if self.entity.physics.vel.x > 0.0 {
                    self.entity.rect.x = tile.x - self.entity.rect.w;
                    self.entity.physics.pos.x = (tile.x - self.entity.rect.w) as f32;
                }
                if self.entity.physics.vel.x < 0.0 {
                    self.entity.rect.x = tile.x + tile.w;
                    self.entity.physics.pos.x = (tile.x + tile.w) as f32;
                }
            }
        }

        self.entity.physics.vel.y += self.entity.physics.acceleration.y + gravity;

        if self.entity.physics.vel.y > 64.0 {
            self.entity.physics.vel.y = 64.0;
        }
        self.entity.physics.pos.y += self.entity.physics.vel.y * dt;
        self.entity.rect.y = self.entity.physics.pos.y as i32;
        self.grounded = false;
        // println!("{:?}", self.entity.physics.vel.y);
        for tile in tiles {
            if self.entity.rect.has_intersection(*tile) {
                if self.entity.physics.vel.y > 0.0 {
                    self.entity.rect.y = tile.y - self.entity.rect.h;
                    self.entity.physics.pos.y = (tile.y - self.entity.rect.h) as f32;
                    self.grounded = true;
                }
                if self.entity.physics.vel.y < 0.0 {
                    self.entity.rect.y = tile.y + tile.h;
                    self.entity.physics.pos.y = (tile.y + tile.h) as f32;
                    self.entity.physics.vel.y = 0.0;
                }
            }
        }
        self.trail.retain_mut(|particle| {
            particle.y +=
                *[-1.0, 1.0].choose(&mut thread_rng()).unwrap() * particle.acceleration.y * dt;
            particle.x +=
                *[-1.0, 1.0].choose(&mut thread_rng()).unwrap() * particle.acceleration.x * dt;
            particle.alpha -= dt * 400.0;
            particle.alpha > 0.0
        });
        self.gun.bullets.retain_mut(|bullet| {
            bullet.x += bullet.acceleration.x * dt * 10.0;
            for tile in tiles {
                if bullet.x < 97.0 && bullet.x > 0.0 && bullet.y > 0.0 && bullet.y < 59.0 {
                    return true;
                } else {
                    return false;
                }
            }
            return true;
        });
        self.entity.animator.animate(dt);

        if self.entity.physics.vel.x == 0.0 {
            self.entity.animator.frame.x = 4;
        } else {
            self.trail_timer += dt;
            if self.trail_timer > 0.025 {
                self.trail.push({
                    let mut p = Particle::new(
                        ((self.entity.rect.x + self.entity.rect.w / 2)
                            - (self.entity.rect.w / 2) * self.entity.physics.dir as i32)
                            as f32,
                        (self.entity.rect.y + (self.entity.rect.h)) as f32,
                    );
                    p.alpha = 160.0;
                    p
                });
                self.trail_timer = 0.0;
            }
        }
    }

    pub fn render<'a>(
        &self,
        display: &mut WindowCanvas,
        cache: &mut HashMap<String, Texture<'a>>,
        offset: (i32, i32),
    ) {
        for particle in &self.gun.bullets {
            display.set_draw_color(Color::RGBA(255, 255, 255, particle.alpha as u8));
            display.draw_point(point!(
                particle.x - offset.0 as f32,
                particle.y - offset.1 as f32
            ));
        }
        display.set_blend_mode(BlendMode::Blend);

        for particle in &self.trail {
            display.set_draw_color(Color::RGBA(255, 255, 255, particle.alpha as u8));
            display.draw_point(point!(
                particle.x - offset.0 as f32,
                particle.y - offset.1 as f32
            ));
        }
        display.set_blend_mode(BlendMode::None);

        text(
            display,
            &self.title,
            Color::RGB(228, 219, 160),
            (self.entity.rect.x + self.entity.rect.w / 2) - offset.0,
            (self.entity.rect.y - 2) - offset.1,
            0.25,
        );
        if self.entity.hp.0 != self.entity.hp.1 {
            display.set_draw_color(Color::WHITE);
            display
                .draw_rect(rect!(
                    self.entity.rect.x - offset.0,
                    self.entity.rect.y - 4 - offset.1,
                    self.entity.hp.1,
                    1
                ))
                .unwrap();
            if !(self.entity.hp.0 <= 0.0) {
                display.set_draw_color(Color::GREEN);

                display
                    .draw_rect(rect!(
                        self.entity.rect.x - offset.0,
                        self.entity.rect.y - 4 - offset.1,
                        self.entity.hp.0,
                        1
                    ))
                    .unwrap();
            }
        }

        display.set_draw_color(Color::RGB(139, 181, 212));
        display.fill_rect(rect!(
            self.entity.rect.x - offset.0,
            self.entity.rect.y - offset.1,
            self.entity.rect.w,
            self.entity.rect.h
        ));
        display
            .copy_ex(
                cache.get_mut("gun").unwrap(),
                None,
                Rect::new(
                    (self.entity.rect.x + 4 * self.entity.physics.dir as i32) - offset.0,
                    ((self.entity.rect.y + self.entity.rect.h / 2) - 1) - offset.1,
                    4,
                    2,
                ),
                0.0,
                None,
                self.entity.physics.dir < 0.0,
                false,
            )
            .unwrap();
        // display.set_blend_mode(BlendMode::Blend);

        // display.set_blend_mode(BlendMode::None);
    }
}

use crate::components::{Animator, Particle, Physics, Stat};
use crate::math::Vec2;
use crate::{rect, vec2};
use rand::prelude::*;
use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};
use std::collections::HashMap;

pub struct Gun {
    pub damage: f32,
    pub bullets: Vec<Particle>,

    pub per_bullet_time: f32,
    pub cool_down_timer: f32,
}
impl Gun {
    pub fn new(damage: f32, per_bullet_time: f32) -> Self {
        Self {
            damage,
            bullets: Vec::new(),
            per_bullet_time,
            cool_down_timer: 0.0,
        }
    }
}

pub struct Entity {
    pub physics: Physics,
    pub rect: Rect,
    pub hp: Stat,
    pub animator: Animator,
}

impl Entity {
    pub fn new(rect: Rect, acceleration: Vec2, hp: f32, animator: Animator) -> Self {
        Self {
            physics: Physics::new(vec2!(rect.x, rect.y), acceleration),
            rect,
            hp: Stat(hp, hp),
            animator,
        }
    }
}

pub struct Wanderer {
    entity: Entity,
    tag: String,
}
impl Wanderer {
    pub fn new(x: i32, y: i32, tag: &str) -> Self {
        let mut accel: f32 = 0.0;
        let mut per_frame_time = 0.0;
        let mut rect = Rect::new(x, y, 1, 1);
        if tag == "caterpillar" {
            accel = thread_rng().gen_range(2.0..4.0);
            per_frame_time = 0.2;
            rect.w = 3;
            rect.h = 2;
        }
        if tag == "caterpillar" {
            accel = thread_rng().gen_range(2.0..4.0);
            per_frame_time = 0.2;
            rect.w = 3;
            rect.h = 2;
        }
        Self {
            entity: Entity::new(
                rect,
                Vec2::new(accel, (rect.w + rect.h) as f32 * 9.8), //vy = mass * grav
                2.0,
                Animator::new(
                    rect!(
                        *[0, rect.w, rect.w * 2, rect.w * 3]
                            .choose(&mut thread_rng())
                            .unwrap(),
                        0,
                        rect.w,
                        rect.h
                    ),
                    3,
                    per_frame_time,
                ),
            ),
            tag: tag.to_string(),
        }
    }
    pub fn update(&mut self, dt: f32, tiles: &Vec<Rect>) {
        self.entity.physics.vel.x = self.entity.physics.dir * self.entity.physics.acceleration.x;
        self.entity.physics.pos.x += self.entity.physics.vel.x * dt;
        self.entity.rect.x = self.entity.physics.pos.x as i32;
        for tile in tiles {
            if self.entity.rect.has_intersection(*tile) {
                if self.entity.physics.vel.x > 0.0 {
                    self.entity.rect.x = tile.x - self.entity.rect.w;

                    self.entity.physics.pos.x = (tile.x - self.entity.rect.w) as f32;
                    self.entity.physics.dir *= -1.0;
                }
                if self.entity.physics.vel.x < 0.0 {
                    self.entity.rect.x = tile.x + tile.w;

                    self.entity.physics.pos.x = (tile.x + tile.w) as f32;
                    self.entity.physics.dir *= -1.0;
                }
            }
        }
        self.entity.physics.vel.y = self.entity.physics.acceleration.y;

        self.entity.physics.pos.y += self.entity.physics.vel.y * dt;
        self.entity.rect.y = self.entity.physics.pos.y as i32;
        for tile in tiles {
            if self.entity.rect.has_intersection(*tile) {
                if self.entity.physics.vel.y > 0.0 {
                    self.entity.rect.y = tile.y - self.entity.rect.h;
                    self.entity.physics.pos.y = (tile.y - self.entity.rect.h) as f32;
                }
                if self.entity.physics.vel.y < 0.0 {
                    self.entity.rect.y = tile.y + tile.h;
                    self.entity.physics.pos.y = (tile.y + tile.h) as f32;
                }
            }
        }
        self.entity.animator.animate(dt);
    }

    pub fn render<'a>(
        &self,
        display: &mut WindowCanvas,
        cache: &mut HashMap<String, Texture<'a>>,
        offset: (i32, i32),
    ) {
        display
            .copy_ex(
                cache.get_mut(&self.tag).unwrap(),
                self.entity.animator.frame,
                rect!(
                    self.entity.rect.x - offset.0,
                    self.entity.rect.y - offset.1,
                    self.entity.rect.w,
                    self.entity.rect.h
                ),
                0.0,
                None,
                self.entity.physics.dir < 0.0,
                false,
            )
            .unwrap();
    }
}

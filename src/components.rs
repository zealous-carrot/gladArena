use crate::math::Vec2;
use crate::rect;
use rand::prelude::*;
use sdl2::{
    rect::Rect,
    render::{Texture, WindowCanvas},
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Particle {
    pub alpha: f32,
    pub acceleration: Vec2,
    pub x: f32,
    pub y: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = thread_rng();
        Self {
            alpha: 255.0,
            acceleration: Vec2::new(rng.gen_range(8.0..16.0), rng.gen_range(8.0..16.0)),
            x,
            y,
        }
    }
}
pub struct Physics {
    pub pos: Vec2,
    pub vel: Vec2,
    pub dir: f32,
    pub acceleration: Vec2,
}
impl Physics {
    pub fn new(pos: Vec2, acceleration: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::new(0.0, 0.0),
            dir: 1f32,
            acceleration,
        }
    }
}

pub struct Stat(pub f32, pub f32);
pub struct Animator {
    pub frame: Rect,
    pub count: i32,
    pub timer: f32,
    pub per_frame_time: f32,
}
impl Animator {
    pub fn new(frame: Rect, count: i32, per_frame_time: f32) -> Self {
        Self {
            frame,
            count,
            timer: 0.0,
            per_frame_time,
        }
    }
    pub fn animate(&mut self, dt: f32) -> bool {
        self.timer += dt;
        if self.timer > self.per_frame_time {
            self.timer = 0.0;
            self.frame.x += self.frame.w;

            if self.frame.x > self.frame.w * (self.count - 1) {
                self.frame.x = 0;
            }
            return true;
        } else {
            return false;
        }
    }
}

pub struct Decor {
    pub pos: (i32, i32),
    pub tag: String,
    pub animator: Animator,
}

impl Decor {
    pub fn new(pos: (i32, i32), tag: String) -> Self {
        let mut animator = Animator::new(rect!(0, 0, 1, 1), 4, 0.1);
        match tag.as_str() {
            "sewage" => {
                animator.frame.w = 8;
                animator.frame.h = 12;
            }
            "fountain" => {
                animator.frame.w = 16;
                animator.frame.h = 20;
            }
            "vine" => {
                animator.frame.w = 10;
                animator.frame.h = 6;
            }

            _ => {}
        }
        Self {
            pos,
            tag: tag,
            animator,
        }
    }
    pub fn render<'a>(
        &self,
        display: &mut WindowCanvas,
        cache: &mut HashMap<String, Texture<'a>>,
        offset: (i32, i32),
    ) {
        display
            .copy(
                cache.get_mut(&self.tag).unwrap(),
                self.animator.frame,
                rect!(
                    self.pos.0 - offset.0,
                    self.pos.1 - offset.1,
                    self.animator.frame.w,
                    self.animator.frame.h
                ),
            )
            .unwrap();
    }
}

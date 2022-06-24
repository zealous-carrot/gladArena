mod components;
mod ecosystem;
mod entities;
mod helpers;
mod input;
mod math;
mod player;
mod ui;

use ecosystem::Ecosystem;
use helpers::load_textures;
use input::Input;
use math::{sine_wave, Vec2};
use rodio::Source;
use rodio::{Decoder, OutputStream};
use sdl2::{pixels::Color, rect::Rect, render::BlendMode};
use std::io::BufReader;
use std::{fs::File, time::Instant};
use ui::*;
#[macro_export]
macro_rules! vec2 {
    ( $x : expr , $y : expr ) => {
        Vec2::new($x as f32, $y as f32)
    };
}
#[macro_export]

macro_rules! rect {
    ( $x : expr , $y : expr,$w : expr , $h : expr ) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}
#[macro_export]

macro_rules! point {
    ( $x : expr , $y : expr ) => {
        Point::new($x as i32, $y as i32)
    };
}

fn main() {
    let mut last = Instant::now();
    let ctx = sdl2::init().unwrap();
    let _img_ctx = sdl2::image::init(sdl2::image::InitFlag::PNG);
    let mut window = {
        let video_subsystem = ctx.video().unwrap();
        video_subsystem
            .window(" ", 776, 472)
            .position_centered()
            .build()
            .unwrap()
    };
    window
        .set_minimum_size(window.size().0, window.size().1)
        .unwrap();
    let mut display = window.into_canvas().present_vsync().build().unwrap();
    let mut zoom = 4.0;
    let mut camera = Vec2::new(
        (display.window().size().0 as f32 / zoom) / 3.0,
        (display.window().size().1 as f32 / zoom) / 3.0,
    );
    display
        .set_logical_size(
            (display.window().size().0 as f32 / zoom) as u32,
            (display.window().size().1 as f32 / zoom) as u32,
        )
        .unwrap();
    let mut texture_creator = display.texture_creator();
    let mut input = Input::new(ctx.event_pump().unwrap());
    let mut cache = load_textures(&mut texture_creator, "res/textures".to_owned());
    let mut layout = Layout::new(97, 59);
    let mut ecosystem = Ecosystem::new();
    let mut current_time = 0.0;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("res/sfx/bg_music.mp3").unwrap());
    let src = Decoder::new(file).unwrap();
    stream_handle.play_raw(src.convert_samples());

    while !input.quit() {
        let new = Instant::now();
        let dt = 1.0 / 60.0;
        last = new;

        input.update();
        display.set_draw_color(layout.bg_color());
        display.clear();
        display
            .set_logical_size(layout.frame().w as u32, layout.frame().h as u32)
            .unwrap();
        match layout.state() {
            0 => {
                //intro
                current_time += dt * 8.0;

                layout.switch(current_time >= 24.0, 1);

                display.set_blend_mode(BlendMode::Blend);
                text(
                    &mut display,
                    "zealous",
                    Color::RGBA(
                        255,
                        255,
                        255,
                        sine_wave(current_time, 50.0, 255.0, 1.0) as u8,
                    ),
                    layout.frame().w / 2,
                    layout.frame().h / 2,
                    0.5,
                );
                let t = cache.get_mut("logo").unwrap();
                t.set_alpha_mod((sine_wave(current_time, 50.0, 255.0, 1.0)) as u8);
                display.copy(
                    t,
                    None,
                    Rect::new(
                        (layout.frame().w as f32 / 1.8) as i32,
                        (layout.frame().h as f32 / 2.5) as i32,
                        7,
                        5,
                    ),
                );
                println!("{:?}", current_time);
            }
            1 => {
                //menu
                text(
                    &mut display,
                    "GladArena",
                    Color::RGB(168, 181, 174),
                    layout.frame().w / 2,
                    layout.frame().h / 4,
                    0.5,
                );
                layout.switch(
                    if button(
                        &mut display,
                        "play",
                        layout.frame_color(),
                        layout.frame().w / 2,
                        layout.frame().h / 2,
                        &input,
                        0.5,
                    ) {
                        ecosystem = Ecosystem::new();
                        true
                    } else {
                        false
                    },
                    3,
                );
                layout.switch(
                    button(
                        &mut display,
                        "credits",
                        layout.frame_color(),
                        layout.frame().w / 2,
                        (layout.frame().h as f32 / 1.5) as i32,
                        &input,
                        0.5,
                    ),
                    2,
                );
            }
            2 => {
                //credits
                layout.switch(
                    button(&mut display, "<", layout.frame_color(), 9, 9, &input, 0.5),
                    1,
                );
                text_ex(
                    &mut display,
                    vec![
                        ("discord: ", Color::RGB(146, 146, 156)),
                        ("zealous_carrot#9124", Color::RGB(241, 234, 182)),
                    ],
                    layout.frame().w / 8,
                    layout.frame().h / 4,
                    0.25,
                );
                text_ex(
                    &mut display,
                    vec![
                        ("github: ", Color::RGB(146, 146, 156)),
                        ("..github/zealous_carrot..", Color::RGB(241, 234, 182)),
                    ],
                    layout.frame().w / 8,
                    layout.frame().h / 3,
                    0.25,
                );
            }
            3 => {
                let win_dim = (
                    (display.window().size().0 as f32 / zoom) as i32,
                    (display.window().size().1 as f32 / zoom) as i32,
                );
                //play
                layout.switch(
                    {
                        // let mut over = false;
                        ecosystem.players[0].is_dead() || ecosystem.players[1].is_dead()
                        // over
                    },
                    5,
                );

                camera.x += (((((ecosystem.players[0].pos().x - ecosystem.players[1].pos().x)
                    .abs()
                    / 2.0)
                    - camera.x)
                    - (win_dim.0 as f32 / 2.0)
                    + 97.0 / 4.0)
                    * dt);
                camera.y += (((((ecosystem.players[0].pos().y - ecosystem.players[1].pos().y)
                    .abs()
                    / 2.0)
                    - camera.y)
                    - (win_dim.1 as f32 / 2.0)
                    + 59.0 / 4.0)
                    * dt);
                display
                    .set_logical_size(
                        (display.window().size().0 as f32 / zoom) as u32,
                        (display.window().size().1 as f32 / zoom) as u32,
                    )
                    .unwrap();
                ecosystem.update(dt, &input, &mut camera);
                ecosystem.render(&mut display, &mut cache, camera);
            }
            4 => {
                //pause

                layout.switch(
                    button(
                        &mut display,
                        "resume",
                        layout.frame_color(),
                        layout.frame().w / 2,
                        layout.frame().h / 4,
                        &input,
                        0.5,
                    ),
                    3,
                );
                layout.switch(
                    button(
                        &mut display,
                        "menu",
                        layout.frame_color(),
                        layout.frame().w / 2,
                        (layout.frame().h / 2) as i32,
                        &input,
                        0.5,
                    ),
                    1,
                );
            }
            5 => {
                //over
                text(
                    &mut display,
                    &format!("{:?} won!", {
                        let mut winner = "Tie";
                        for player in &ecosystem.players {
                            if player.entity.hp.0 > 0.0 {
                                winner = player.title();
                            }
                        }
                        winner
                    }),
                    Color::RGB(241, 234, 182),
                    layout.frame().w / 2,
                    layout.frame().h / 4,
                    0.5,
                );
                layout.switch(
                    if button(
                        &mut display,
                        "retry",
                        layout.frame_color(),
                        layout.frame().w / 2,
                        (layout.frame().h as f32 / 2.5) as i32,
                        &input,
                        0.5,
                    ) {
                        ecosystem = Ecosystem::new();
                        true
                    } else {
                        false
                    },
                    3,
                );
                layout.switch(
                    button(
                        &mut display,
                        "menu",
                        layout.frame_color(),
                        layout.frame().w / 2,
                        (layout.frame().h as f32 / 1.5) as i32,
                        &input,
                        0.5,
                    ),
                    1,
                );
            }
            _ => {}
        }
        layout.render(&mut display);

        display.present();
    }
}

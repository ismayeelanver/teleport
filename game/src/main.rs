use std::time::Instant;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use utils::{self, HEIGHT, WIDTH};
fn main() -> anyhow::Result<(), String> {
    let sdl2_context = sdl2::init()?;
    let video_renderer = sdl2_context.video()?;
    let window = video_renderer
        .window("Game", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Could not initialize the video renderer");

    let mut cs = window
        .into_canvas()
        .build()
        .expect("Could not initialize a canvas");

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    cs.clear();
    cs.present();

    let mut eh = sdl2_context.event_pump()?;
    let start = Instant::now();
    'running: loop {
        let elapsed = Instant::now().duration_since(start).as_secs_f32();
        let t = (elapsed / 5.0).fract();
        r = (1.0 * (t * 2.0 * std::f32::consts::PI).sin() * 255.0) as u8;
        g = (1.0 * ((t + 0.33) * 2.0 * std::f32::consts::PI).sin() * 255.0) as u8;
        b = (1.0 * ((t + 0.07) * 2.0 * std::f32::consts::PI).sin() * 255.0) as u8;
        if r == 255 {
            r = ((elapsed as f32 * 0.1) % -1.0 * 255.0) as u8
        }
        cs.clear();
        cs.set_draw_color(Color::RGB(r, g, b));
        cs.present();
        for event in eh.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

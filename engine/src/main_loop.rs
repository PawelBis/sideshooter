use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use thiserror::Error;

/// Wrapper for sdl errors
#[derive(Error, Debug)]
pub enum SdlError {
    #[error("general sdl error")]
    General(String),
    #[error("failed to build window: {0}")]
    BuildWindow(#[from] sdl2::video::WindowBuildError),
    #[error("integer or sdl error: {0}")]
    IntegerOrSdl(#[from] sdl2::IntegerOrSdlError),
}

pub fn run() -> Result<(), SdlError> {
    let sdl_context = sdl2::init().map_err(SdlError::General)?;
    let video_subsystem = sdl_context.video().map_err(SdlError::General)?;

    let window = video_subsystem
        .window("Sideshooter", 800, 600)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().map_err(SdlError::General)?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }

    Ok(())
}

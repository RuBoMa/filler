use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <game_log.txt>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("Failed to open file {}: {}", filename, err);
        std::process::exit(1);
    });
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let first_line = lines.first().cloned().unwrap_or_else(|| "Log file is empty".to_string());

    // SDL2 Initialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("Visualizer Test", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    // Load font
    let font_path = "assets/Roboto-Regular.ttf";
    let font: Font = ttf_context.load_font(font_path, 24).unwrap();

    let surface = font
        .render(&first_line)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    let texture_query = texture.query();
    let target = Rect::new(50, 50, texture_query.width, texture_query.height);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the text
        canvas.copy(&texture, None, Some(target)).unwrap();

        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
}

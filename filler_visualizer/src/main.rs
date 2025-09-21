mod field;
mod piece;
mod player;
mod grid;
mod visualizer;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <game_log.txt>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let lines = read_lines_from_file(filename);
    let mut visualizer = visualizer::Visualizer::new(lines);

    // SDL2 Initialization
    let (mut canvas, mut event_pump, ttf_context) = init_sdl2("Visualizer", 1200, 900);

    
    // Create texture creator for font rendering
    let texture_creator = canvas.texture_creator();

    // Load font once
    let font_path = "assets/Roboto-Regular.ttf";
    let font = ttf_context.load_font(font_path, 12).unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                // Navigation: Left/right arrow to move between turns
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => visualizer.next_turn(),

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => visualizer.prev_turn(),

                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        visualizer.draw(&mut canvas, &font, &texture_creator);

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}

use std::io;
fn read_lines_from_file(filename: &str) -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("Failed to open file {}: {}", filename, err);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);
    reader.lines()
}

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::ttf::Sdl2TtfContext;

fn init_sdl2(
    title: &str,
    width: u32,
    height: u32,
) -> (Canvas<Window>, EventPump, Sdl2TtfContext) {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to get video subsystem");
    let ttf_context = sdl2::ttf::init().expect("Failed to initialize TTF");

    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Failed to create canvas");

    let event_pump = sdl_context
        .event_pump()
        .expect("Failed to create event pump");

    (canvas, event_pump, ttf_context)
}
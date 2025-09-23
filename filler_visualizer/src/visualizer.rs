pub use crate::field::*;
pub use crate::piece::*;
pub use crate::player::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub struct Visualizer {
    pub players: [Player; 2],
    pub fields: Vec<Field>,
    pub pieces: Vec<(usize, Piece)>,
    pub turn: usize,
}

impl Visualizer {
    pub fn new<I>(mut lines: I) -> Self
    where
        I: Iterator<Item = Result<String, std::io::Error>>,
    {
        let player1_line = lines
            .next()
            .expect("Missing line for player 1")
            .expect("Failed to read player 1 line");

        let player2_line = lines
            .next()
            .expect("Missing line for player 2")
            .expect("Failed to read player 2 line");

        let player1 = Player::new(&player1_line);
        let player2 = Player::new(&player2_line);

        let mut fields = Vec::new();
        let mut pieces = Vec::new();

        while let Some(Ok(line)) = lines.next() {
            if line.starts_with("Anfield") {
                let mut field = Field::new(&line);
                field.update(&mut lines);
                fields.push(field);

            } else if line.starts_with("Piece") {
                let mut piece = Piece::new(&line);
                piece.update(&mut lines);

                let answer_line = lines.next().unwrap().unwrap();
                let symbol = answer_line.chars().nth(11).unwrap_or(' ');

                let player = match symbol {
                    '@' => 1,
                    '$' => 2,
                    _ => 0,
                };

                pieces.push((player, piece));
            }
        }

        Visualizer {
            players: [player1, player2],
            fields,
            pieces,
            turn: 0,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        let field = &self.fields[self.turn];
        let max_grid_size = 800;

        let cell_size = (max_grid_size / field.size.width.max(1))
                                .min(max_grid_size / field.size.height.max(1));
        let margin = 1;

        let mut p1_score = 0;
        let mut p2_score = 0;
        let p1_col = Color::RGB(255, 100, 100);
        let p2_col = Color::RGB(100, 100, 255);

        // Draw field grid at (50, 50)
        for (row_idx, row) in field.cells.iter().enumerate() {
            for (col_idx, &ch) in row.iter().enumerate() {
                let x = 50 + col_idx as u32 * cell_size as u32;
                let y = 50 + row_idx as u32 * cell_size as u32;
                
                match ch {
                    '@' | 'a' => p1_score += 1,
                    '$' | 's' => p2_score += 1,
                    _ => {}
                }

                let rect = Rect::new(
                    x as i32 + margin,
                    y as i32 + margin,
                    (cell_size as u32).saturating_sub(2 * margin as u32),
                    (cell_size as u32).saturating_sub(2 * margin as u32),
                );
                let color = match ch {
                    '@' => Color::RGB(150, 60, 60),
                    'a' => Color::RGB(255, 100, 100),
                    '$' => Color::RGB(60, 60, 150),
                    's' => Color::RGB(100, 100, 255),
                    '.' => Color::RGB(40, 40, 40),
                    _ => Color::RGB(80, 80, 80),
                };

                canvas.set_draw_color(color);
                canvas.fill_rect(rect).ok();
            }
        }

        // Draw text: Player info & Turn
        draw_text(canvas, texture_creator, font,
            &format!("Turn: {} / {}", self.turn + 1, self.fields.len()),
            880, 50, Color { r: 255, g: 255, b: 255, a: 0 });

        draw_text(canvas, texture_creator, font,
            &format!("Player 1: {}", self.players[0].path),
            880, 110, p1_col);
        draw_text(canvas, texture_creator, font,
            &format!("   score: {}", p1_score),
            880, 140, p1_col);

        draw_text(canvas, texture_creator, font,
            &format!("Player 2: {}", self.players[1].path),
            880, 170, p2_col);
        draw_text(canvas, texture_creator, font,
            &format!("   score: {}", p2_score),
            880, 200, p2_col);


        // Draw piece grid (100x100 max size)
        if let Some((player, piece)) = self.pieces.get(self.turn) {
            let px = 880;
            let py = 250;

            let col = match player {
                1 => p1_col,
                2 => p2_col,
                _ => Color { r: 255, g: 255, b: 255, a: 0 },
            };

            draw_text(canvas, texture_creator, font,
                &format!("Player {} placing: ", player),
                880, py, col);

            let cell_size = (150 / piece.size.width.max(1))
                                .min(150 / piece.size.height.max(1)).min(50);
            let margin = 1;

            for (row_idx, row) in piece.cells.iter().enumerate() {
                for (col_idx, &ch) in row.iter().enumerate() {

                    let x = px + col_idx as i32 * cell_size as i32;
                    let y = py + 30 + row_idx as i32 * cell_size as i32;

                    let rect = Rect::new(
                        x + margin,
                        y + margin,
                        (cell_size as u32).saturating_sub(2 * margin as u32),
                        (cell_size as u32).saturating_sub(2 * margin as u32),
                    );

                    let color = match ch {
                        'O' => col,
                        _   => Color::RGB(50, 50, 50),
                    };

                    canvas.set_draw_color(color);
                    canvas.fill_rect(rect).ok();
                }
            }
        }
    }
    
    pub fn next_turn(&mut self) {
        if self.turn + 1 < self.fields.len() {
            self.turn += 1;
        }
    }

    pub fn prev_turn(&mut self) {
        if self.turn > 0 {
            self.turn -= 1;
        }
    }

    pub fn first_turn(&mut self) {
        self.turn = 0;
    }

    pub fn last_turn(&mut self) {
        self.turn = self.fields.len() -1;
    }

}

fn draw_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    text: &str,
    x: i32,
    y: i32,
    color: Color,
) {
    let surface = font
        .render(text)
        .blended(color)
        .unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let target = Rect::new(x, y, surface.width(), surface.height());
    canvas.copy(&texture, None, Some(target)).unwrap();
}

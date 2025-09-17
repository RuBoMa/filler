use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read player number line
    let _first_line = lines.next().unwrap().unwrap();

    loop {
        // Read Anfield header (e.g. "Anfield 20 15:")
        let anfield_info = match lines.next() {
            Some(Ok(line)) if line.starts_with("Anfield") => line,
            _ => break,
        };

        let parts: Vec<&str> = anfield_info.split_whitespace().collect();
        let rows: usize = parts[1].parse().unwrap();

        // Read Anfield grid
        for _ in 0..rows {
            let _line = lines.next();
        }

        // Read Piece header
        let piece_info = lines.next().unwrap().unwrap();
        let piece_parts: Vec<&str> = piece_info.split_whitespace().collect();
        let piece_rows: usize = piece_parts[1].parse().unwrap();

        // Read Piece grid
        for _ in 0..piece_rows {
            let _line = lines.next();
        }

        
        println!("0 0");
    }
}

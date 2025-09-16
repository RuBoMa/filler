use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (player_char, last_piece_char) = if first_line.contains("p2") {
        ('$','s')
    } else {
        ('@','a')
    };
    println!("Player chars: {} {}", player_char, last_piece_char);

    let anfield_info = lines.next().unwrap().unwrap();
    let parts: Vec<&str> = anfield_info.split_whitespace().collect();
    let rows: usize = parts[1].parse().unwrap();
    let cols: usize = parts[2].trim_end_matches(':').parse().unwrap();
    println!("Anfield size: {} rows x {} cols", rows, cols);

    let mut anfield: Vec<Vec<char>> = Vec::new();
    for _ in 0..rows {
        let line = lines.next().unwrap().unwrap();
        let row_data: Vec<char> = line[4..].chars().collect();
        anfield.push(row_data);
    }

    for row in &anfield {
        println!("{}", row.iter().collect::<String>());
    }

    println!("0 0");
}

use std::fs::File;
use std::io::{BufRead, BufReader};

fn pretty_print(dg: Vec<Vec<char>>) {
    for line in dg {
        for ch in line {
            print!("{}", ch);
        }
        print!("\n");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);

    let mut diagram: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        diagram.push(line?.chars().collect());
    }
    let line_length: usize = diagram[0].len();
    let mut splitter_count: usize = 0;
    for line in 1..diagram.len() {
        let mut ac: Vec<char> = Vec::new();
        for i in 0..line_length {
            ac.push(diagram[line - 1][i]);
        }
        // println!("dg {:?}", diagram);
        // println!("ac {:?}", ac);

        for (col, ch) in ac.into_iter().enumerate() {
            if ch == 'S' || ch == '|' {
                // laser hiting empty space (continue)
                if diagram[line][col] == '.' {
                    diagram[line][col] = '|';
                }
                // laser hiting a splitter (split to left and rigth)
                if diagram[line][col] == '^' {
                    splitter_count += 1;
                    if col + 1 < line_length {
                        diagram[line][col + 1] = '|'
                    }
                    if col > 0 {
                        diagram[line][col - 1] = '|'
                    }
                }
            }
        }
    }

    pretty_print(diagram);
    println!("splitter_count -> {}", splitter_count);
    Ok(())
}

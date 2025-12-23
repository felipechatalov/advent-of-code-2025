use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_roll(diagram: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    return diagram[i][j] == '@';
}

fn nearby_rolls(diagram: &Vec<Vec<char>>, i: i32, j: i32) -> usize {
    let mut total: usize = 0;
    let max_col = diagram.len().try_into().unwrap();
    let max_lines = diagram[0].len().try_into().unwrap();

    for line in i - 1..i + 2 {
        for col in j - 1..j + 2 {
            if line < 0 || col < 0 || (line == i && col == j) || line >= max_lines || col >= max_col
            {
                continue;
            }
            if is_roll(
                &diagram,
                line.try_into().expect("to usize"),
                col.try_into().expect("to usize"),
            ) {
                total += 1;
            }
        }
    }
    return total;
}

fn get_rolls(diagram: &Vec<Vec<char>>) -> usize {
    let mut acessible: usize = 0;
    for i in 0..diagram.len() {
        for j in 0..diagram[0].len() {
            if is_roll(&diagram, i, j)
                && nearby_rolls(
                    &diagram,
                    i.try_into().expect("to i32"),
                    j.try_into().expect("to i32"),
                ) < 4
            {
                acessible += 1;
            }
        }
    }
    return acessible;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut diagram: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let diagram_line: Vec<char> = line?.chars().collect();
        diagram.push(diagram_line);
    }

    let rolls_acessible = get_rolls(&diagram);
    println!("rolls_acessible -> {}", rolls_acessible);

    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader};

fn abs(n: i64) -> u64 {
    if n < 0 {
        return (n * -1) as u64;
    }
    return n as u64;
}

fn uminus(n1: u64, n2: u64) -> u64 {
    return abs(n1 as i64 - n2 as i64) + 1;
}

fn area(a: &Point, b: &Point) -> u64 {
    return uminus(a.x, b.x) * uminus(a.y, b.y);
}

struct Point {
    x: u64,
    y: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Point> = Vec::new();
    for line in reader.lines() {
        let val: Vec<u64> = line?
            .split(',')
            .map(|a| a.parse::<u64>().expect("parse error"))
            .collect();
        grid.push(Point {
            x: val[0],
            y: val[1],
        });
    }

    let length: usize = grid.len();

    let mut big_area: u64 = 0;
    for i in 0..length - 1 {
        for j in i + 1..length {
            let rect: u64 = area(&grid[i], &grid[j]);
            // println!(
            //     "({},{})-({},{}) -> {}",
            //     grid[i].x, grid[i].y, grid[j].x, grid[j].y, rect
            // );
            if rect > big_area {
                big_area = rect;
            }
        }
    }

    println!("biggest_area {:?}", big_area);

    Ok(())
}

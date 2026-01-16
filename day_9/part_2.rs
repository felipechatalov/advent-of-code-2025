use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_valid_rect(a: &Point, b: &Point) -> bool {
    return false;

    // 'dir' says where the rect grows into, like a vector pointing in diagonal from point a to b
    // where:
    //   x+->
    // y+
    // |
    // v
    //        ^ false
    // false < > true
    //        v true
    let dir: (bool, bool) = (false, false);
    if a.x < b.x {
        dir.0 = true;
    }
    if a.y > b.y {
        dir.1 = true;
    }

    let unkp1: Point = Point { x: a.x, y: b.y };
    let unkp2: Point = Point { x: b.x, y: a.y };

    let unkp1_valid: bool = false;
    let unkp2_valid: bool = false;

    // not gonna work because the polygon might be convex shaped :c
    for p in &points {
        // 'a.x < b.x' so we know that x is valid, we need to check y. y valid if point.y <= unkown point.x
        if dir.1 && p.y >= unkp1.y && dir.0 && p.x <= unkp1.y {
            unkp1_valid = true;
        }
    }

    // after knowing the direction the rect goes, we need to know if there is any point that is
    // 'further' or equal then it, where further depends on the direction
    // if both unknown vertices is 'closer' or equal another know point, it is valid
}

fn abs(n: i64) -> u64 {
    if n < 0 {
        return (n * -1) as u64;
    }
    return n as u64;
}

fn uminus(n1: usize, n2: usize) -> u64 {
    return abs(n1 as i64 - n2 as i64) + 1;
}

fn area(a: &Point, b: &Point) -> u64 {
    return uminus(a.x, b.x) as u64 * uminus(a.y, b.y) as u64;
}

struct Point {
    x: usize,
    y: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut points: Vec<Point> = Vec::new();
    for line in reader.lines() {
        let val: Vec<usize> = line?
            .split(',')
            .map(|a| a.parse::<usize>().expect("parse error"))
            .collect();
        points.push(Point {
            x: val[0],
            y: val[1],
        });
    }

    let length: usize = points.len();

    let grid: Vec<Vec<usize>> = Vec::new();
    println!("start");
    let grid = make_grid(&points);
    println!("finish");

    let mut big_area: u64 = 0;
    for i in 0..length - 1 {
        for j in i + 1..length {
            if !is_valid_rect(&points[i], &points[j]) {
                continue;
            }
            let rect: u64 = area(&points[i], &points[j]);
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

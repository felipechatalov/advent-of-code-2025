use std::fs::File;
use std::io::{BufRead, BufReader};

fn merge(a: usize, b: usize, arr: &mut Vec<Vec<usize>>) {
    let mut bci: usize = 0;
    let mut aci: usize = 0;

    for idx in 0..arr.len() {
        if arr[idx].contains(&a) {
            aci = idx;
        }
        if arr[idx].contains(&b) {
            bci = idx;
        }
    }

    for idx in 0..arr[bci].len() {
        let val = arr[bci][idx];
        arr[aci].push(val);
    }

    arr.remove(bci);
    return;
}

fn connect(a: usize, b: usize, arr: &mut Vec<Vec<usize>>) {
    for conn in &mut *arr {
        if conn.contains(&a) && conn.contains(&b) {
            // println!("already same circuit");
            return;
        }
    }

    if isconnected(a, arr) && isconnected(b, arr) {
        // println!("merging {} and {} circuits", a, b);
        merge(a, b, arr);
        return;
    }

    if isconnected(a, arr) {
        for conn in &mut *arr {
            if conn.contains(&a) {
                conn.push(b);
                return;
            }
        }
    }
    if isconnected(b, arr) {
        for conn in &mut *arr {
            if conn.contains(&b) {
                conn.push(a);
                return;
            }
        }
    }
    let new_conn: Vec<usize> = vec![a, b];
    arr.push(new_conn);
    return;
}

fn isconnected(a: usize, arr: &Vec<Vec<usize>>) -> bool {
    for conn in arr {
        if conn.contains(&a) {
            return true;
        }
    }
    return false;
}

fn square(a: f64) -> f64 {
    return a * a;
}

fn abs(a: f64) -> f64 {
    if a > 0.0 {
        return a;
    }
    return a * -1.0;
}

fn distance(a: &Vec<u32>, b: &Vec<u32>) -> f64 {
    let x: f64 = abs(a[0] as f64 - b[0] as f64);
    let y: f64 = abs(a[1] as f64 - b[1] as f64);
    let z: f64 = abs(a[2] as f64 - b[2] as f64);

    return (square(x) + square(y) + square(z)).sqrt();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut boxes: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let coord: Vec<u32> = line?
            .split(",")
            .map(|x| x.parse::<u32>().expect("parse u32 error"))
            .collect();
        boxes.push(coord);
    }

    let length: usize = boxes.len();
    let mut connections: Vec<Vec<usize>> = Vec::new();
    #[derive(Debug, Clone)]
    struct Pair {
        src: usize,
        dst: usize,
        dist: f64,
    }
    let mut distances: Vec<Pair> = Vec::new();

    for i in 0..length - 1 {
        for j in i + 1..length {
            distances.push(Pair {
                src: i,
                dst: j,
                dist: distance(&boxes[i], &boxes[j]),
            });
        }
    }
    distances.sort_by(|a, b| a.dist.total_cmp(&b.dist));
    distances = distances.split_at(1000).0.to_vec();

    for pair in distances {
        connect(pair.src, pair.dst, &mut connections);
    }

    connections.sort_by(|a, b| b.len().cmp(&a.len()));
    let ans: usize = connections
        .split_at(3)
        .0
        .into_iter()
        .map(|a| a.len())
        .product();
    println!("ans {:?}", ans);

    Ok(())
}

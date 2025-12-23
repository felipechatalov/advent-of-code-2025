use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut problems: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<String> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        if i < 4 {
            let l = line?;
            let mut inner: Vec<u64> = Vec::new();
            for (i, number) in l.split(" ").enumerate() {
                if number == "" {
                    continue;
                }
                let unum = number.parse::<u64>();
                println!("unum '{}'", number);
                inner.push(unum?);
            }
            problems.push(inner);
        }
    }

    println!("problems -> {:?}", problems);

    let ans: u64 = 0;
    println!("ans -> {}", ans);
    Ok(())
}

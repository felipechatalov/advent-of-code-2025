use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<Vec<u64>> = Vec::new();
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
            numbers.push(inner);
        } else {
            let l = line?;
            for (i, op) in l.split(" ").enumerate() {
                if op == "" {
                    continue;
                }
                operations.push(op.to_string());
            }
        }
    }

    println!("numbers -> {:?}", numbers);
    println!("operations -> {:?}", operations);

    let mut ans: u64 = 0;
    for i in 0..numbers[0].len() {
        if operations[i] == "*" {
            ans += numbers[0][i] * numbers[1][i] * numbers[2][i] * numbers[3][i];
        }
        if operations[i] == "+" {
            ans += numbers[0][i] + numbers[1][i] + numbers[2][i] + numbers[3][i];
        }
    }

    println!("ans -> {}", ans);
    Ok(())
}

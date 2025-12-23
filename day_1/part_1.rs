use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut dial: i32 = 50;
    let mut res: i32 = 0;

    for line in reader.lines() {
        let line = line?;

        let mut dir = 1;
        if line.chars().nth(0) == Some('L') {
            dir = -1;
        }

        let mut amount_str = line;
        amount_str.remove(0);
        let amount: i32 = amount_str.parse().unwrap();

        dial += dir * (amount % 100);
        if dial < 0 {
            dial += 100;
        }
        dial = dial % 100;

        if dial == 0 {
            res += 1;
        }
    }
    println!("res->{}", res);
    Ok(())
}

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

        let mut amount_str = line.clone();
        amount_str.remove(0);
        let amount: i32 = amount_str.parse().unwrap();

        let mut points = 0;
        // let dial_start = dial;

        points += amount / 100;
        let remainder = amount % 100;

        for _ in 0..remainder.abs() {
            dial += dir;
            if dial == 0 {
                points += 1;
            } else if dial >= 100 {
                points += 1;
                dial -= 100;
            } else if dial < 0 {
                dial += 100;
            }
        }

        res += points;

        // println!(
        //     "Starting at {}. Dial is rotated '{}' to point at {}",
        //     dial_start, line, dial
        // );
        // if points > 0 {
        //     println!("During this rotation, it points at 0 {} times", points);
        // }
    }
    println!("res->{}", res);
    Ok(())
}

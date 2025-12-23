use std::fs::File;
use std::io::{BufRead, BufReader};

fn max_joltage_from_bank(bank: &str) -> usize {
    let mut max_joltage: usize = 0;

    let length = bank.len();
    for i in 0..length {
        for j in i..length {
            if i == j {
                continue;
            }

            let mut value = String::from("");
            value.push(bank.chars().nth(i).expect("none"));
            value.push(bank.chars().nth(j).expect("none"));
            let number: usize = value.parse().expect("err");
            if number > max_joltage {
                max_joltage = number;
            }
        }
    }
    return max_joltage;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut res: usize = 0;

    for line in reader.lines() {
        let battery_bank = line?;

        println!("battery_bank -> {:?}", battery_bank);

        let bank_joltage = max_joltage_from_bank(&battery_bank);
        res += bank_joltage;

        println!("max-> {} - {:?}", bank_joltage, battery_bank);
    }

    println!("res -> {}", res);

    Ok(())
}

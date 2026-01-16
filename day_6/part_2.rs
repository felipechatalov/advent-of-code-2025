use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_num_at(numbers: &Vec<Vec<char>>, index: usize) -> u64 {
    let mut candidate: String = "".to_string();
    for i in 0..4 {
        if numbers[i][index] == ' ' {
            continue;
        }
        candidate.push(numbers[i][index]);
    }

    // println!("candidate -> {}", candidate);
    return candidate.parse::<u64>().expect("tou64");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let l = line?;
        numbers.push(l.chars().collect());
    }
    let line_length: usize = numbers[0].len();

    // println!("numbers -> {:?}", numbers);

    let mut ans: u64 = 0;

    let mut cursor: usize = 0;
    while cursor < line_length {
        // 0-3 numbers collumns
        // 4 operations

        let operation: char = numbers[4][cursor];
        let mut parsed: Vec<u64> = Vec::new();
        cursor += 1;
        while cursor == line_length || (cursor < line_length && numbers[4][cursor] == ' ') {
            let mut candidate: String = "".to_string();
            for i in 0..4 {
                if numbers[i][cursor - 1] == ' ' {
                    continue;
                }
                candidate.push(numbers[i][cursor - 1]);
            }
            println!("candidate -> {}", candidate);
            if candidate != "" {
                let cand_parsed = candidate.parse::<u64>().expect("tou64");
                parsed.push(cand_parsed);
            }

            cursor += 1;
        }
        println!("parsed -> {:?}", parsed);
        let mut r: u64 = 0;
        if operation == '+' {
            for number in &parsed {
                r += number;
            }
        }
        if operation == '*' {
            r = 1;
            for number in &parsed {
                r *= number;
            }
        }

        print!("\n");
        for number in &parsed {
            print!("{}, ", number);
        }
        print!("{:?} == {}\n", operation, r);

        ans += r;
    }

    print!("\n");
    println!("ans -> {}", ans);
    Ok(())
}

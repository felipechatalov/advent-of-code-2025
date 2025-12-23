use std::fs::File;
use std::io::{BufRead, BufReader};

fn max_joltage_from_bank(bank: &str) -> u64 {
    let length = bank.len();

    let mut bankc = Vec::new();
    for ch in bank.chars() {
        bankc.push(ch);
    }

    return max_in_line(bankc, 0, length, 12);
}

// example with battery length of 5
// first we place the candidates, starting at digits equal to '9'
// 1957280375839298275482049
// _^__________^_^_______~~~ <- we skip the last 'length-1' since they cant create valid ones
//
// then, we look for the next digits beginning on each candidate start. In this case:
//
// * for the first candidate *
// 1957280375839298275482049
// _.__________^_^_______~~~
//

// for length = 5
// 1957280375839298275482049
// _1___________________~~~~

// 1957280375839298275482049
// _1__________2_________~~~

// 1957280375839298275482049
// _1__________2_3________~~

// 1957280375839298275482049
// _1__________2_34________~

// 1957280375839298275482049
// _1__________2_34________5

// fn get_max_in(bank: &Vec<char>, start: usize, end: usize){
//
// }

// going from left to right, find the highest number
// then go from the last hight number to the end of the vector, ignoring the last few because
// wouldnt make a valid number
// do this 12 times

fn max_in_line(bank: Vec<char>, start: usize, bank_length: usize, word_length: usize) -> u64 {
    let mut start_at = start;
    let mut final_string: String = "".to_string();

    for n in 0..word_length {
        let mut found: bool = false;
        let mut highest: u32 = 9;
        while !found {
            //0..15-12+0 -> 0, 1, 2
            for i in start_at..bank_length - word_length + n + 1 {
                if bank.get(i).expect("notchar").to_digit(10).expect("why") >= highest {
                    final_string.push(bank.get(i).expect("han").clone());
                    start_at = i + 1;
                    highest = 10;
                    found = true;
                    break;
                }
            }
            highest -= 1;
        }
        // get_highest_and_append(bank, start, end, buf);
    }

    println!("bank -> {:?}", bank);
    println!("final -> {:?}", final_string);

    return final_string.parse::<u64>().expect("nice");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut res: u64 = 0;

    for line in reader.lines() {
        let battery_bank = line?;

        println!("battery_bank -> {:?}", battery_bank);

        let bank_joltage = max_joltage_from_bank(&battery_bank);
        res += bank_joltage;

        println!("max-> {} from {:?}", bank_joltage, battery_bank);
    }

    println!("res -> {}", res);

    Ok(())
}

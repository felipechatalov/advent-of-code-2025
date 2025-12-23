use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_even(num: usize) -> bool {
    return num % 2 == 0;
}

fn check_duplicate(product_id: &str) -> bool {
    // println!("product {}", product_id);
    let length: usize = product_id.len();
    if !is_even(length) {
        return false;
    }

    for idx in 0..length / 2 {
        if product_id.chars().nth(idx as usize)
            != product_id.chars().nth((length / 2) + idx as usize)
        {
            // println!(
            //     "Wrong at {}/{} - expected {:?}, got {:?}",
            //     idx,
            //     length / 2 + idx,
            //     product_id.chars().nth(idx as usize),
            //     product_id.chars().nth((length / 2) + idx as usize)
            // );
            return false;
        }
    }

    return true;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line: String = "".to_string();

    _ = reader.read_line(&mut line);

    line.remove(line.len() - 1);
    let mut holder: i64 = 0;
    for line in line.split(",") {
        println!("{:?}", line);
        let mut hold = line.split("-");
        let start = hold.nth(0);
        let end = hold.nth(0);
        println!("start {:?}, end {:?}", start, end);
        let starti: i64 = start.expect("WAS NONE").parse().unwrap();
        let endi: i64 = end.expect("WAS NONE").parse().unwrap();
        println!("starti {}, endi {}", starti, endi);

        for number in starti..endi + 1 {
            let number_str = number.to_string();
            if check_duplicate(&number_str) {
                println!("Fake product -> {}, range->{:?}-{:?}", number, start, end);
                holder += number;
            }
        }
    }
    println!("holder -> {}", holder);
    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader};
fn check_fake_product(product_id: &str) -> bool {
    let length: usize = product_id.len();
    let wstart: usize = 0;
    let mut wend: usize;

    // only a fake product if its make of substrings of at least 2 characters
    // and only composed of those substrings

    // for-loop create the substring
    // println!("for {:?}, {:?}", product_id, 2..length / 2);
    for size in 1..length / 2 + 1 {
        if length % size != 0 {
            continue;
        }
        wend = wstart + size;
        let substring = &product_id[wstart..wend];
        // println!(
        //     "Substring {:?}, size {}, wstart {}, wend {}",
        //     substring, size, wstart, wend
        // );

        // for-loop through the rest of the id, like 10'21'02, then 102'102
        let mut matches: bool = true;
        for substart in (wend..length - size + 1).step_by(size).collect::<Vec<_>>() {
            // println!("to print {:?}", to_print);

            if *substring != product_id[substart..(substart + size)] {
                // let to_print = &product_id[substart..(substart + size)];
                // println!(
                //     "Miss repeating {:?} in {:?}",
                //     substring,
                //     &product_id[substart..(substart + size)]
                // );
                matches = false;
            }
        }
        if matches == true {
            return true;
        }
    }
    return false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input2.txt")?;
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
        // println!("start {:?}, end {:?}", start, end);
        let starti: i64 = start.expect("WAS NONE").parse().unwrap();
        let endi: i64 = end.expect("WAS NONE").parse().unwrap();
        // println!("starti {}, endi {}", starti, endi);

        for number in starti..endi + 1 {
            let number_str = number.to_string();
            if check_fake_product(&number_str) {
                println!("Fake product -> {}, range->{:?}-{:?}", number, start, end);
                holder += number;
            }
        }
    }
    println!("holder -> {}", holder);
    Ok(())
}

use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_between(id: &str, start: &str, end: &str) -> bool {
    let id_len: usize = id.len();
    let start_len: usize = start.len();

    if id_len != start_len {
        return false;
    }

    // 15903 is within
    // 15824 - 15918 ?
    //
    // we put then side by side and compare each most significant digit at a time
    // 15903 id
    // 15824 start
    // 15918 end
    // ^____ comparing the first digit should be -> false if 'id' < start || 'id' > end
    // by making that, we can discard this digit because its no significant anymore to the
    // comparison
    //
    // _5903 id
    // _5824 start
    // _6918 end
    // _^___ now we repeat the same process
    //
    // if the condition is true, than the number is out of the range, if no comparion is true then
    // the number is witin the range

    if lower(id, end) && higher(id, start) {
        return true;
    }
    // println!("{:?} is out of {:?}-{:?} range", id, start, end,);
    return false;
}

fn lower(id: &str, end: &str) -> bool {
    return id.parse::<u64>().unwrap() < end.parse::<u64>().unwrap();
}

fn higher(id: &str, start: &str) -> bool {
    return id.parse::<u64>().unwrap() > start.parse::<u64>().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<String> = Vec::new();
    let mut ids: Vec<String> = Vec::new();
    let mut is_ranges = true;
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            is_ranges = false;
        }

        if is_ranges {
            ranges.push(l);
        } else {
            ids.push(l);
        }
    }

    // println!("ranges_len -> {}", ranges.len());
    // println!("ids_len -> {}", ids.len());

    let mut fresh_foods: usize = 0;
    for food in ids.clone().into_iter() {
        for range in ranges.clone().into_iter() {
            let mut splits = range.split('-');
            if is_between(
                &food,
                splits.next().expect("first"),
                splits.next().expect("second"),
            ) {
                fresh_foods += 1;
                break;
            }
        }
    }
    println!("fresh_foods -> {}", fresh_foods);
    Ok(())
}

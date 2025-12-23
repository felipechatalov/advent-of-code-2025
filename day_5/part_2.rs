use std::fs::File;
use std::io::{BufRead, BufReader};

// 1-2 -> 1, 2
// 3-8 -> 3, 4, 5, 6, 7, 8
// 5-15 -> 9, 10, 11, 12, 13, 14, 15
// 9-13 ->
// 17-20 -> 17, 18, 19, 20
//
// 1, 3, 5, 9, 17
// 2, 8, 15, 13, 20
// sorted -> 2, 8, 13, 15, 20
//
//1-2 -> 1, 2
//3-15 -> 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
//we skip start of 5 cuz its between 3 and 8, so we change our end to the 5-end
//also, we see that 9 is < than 15, which is our new end, so we dicard it, as well as its end-pair,
//cause it is < than our current end
//17-20 -> 17, 18, 19, 20
//
// 3-5
// 10-14
// 16-20
// 12-18
//
//3-5
//10-14 -> 10-18(discard 12) -> 10-20(discard 16)
//10-20
//
//
// place all start in a array, as well as all the ends in other array.
// sort them both
// then we can get 1 from each to be the closed range candidate, where we can iterate over it
// without fear of overlapping with the start or end of another one

// no garantee that the pair will remaing in the same index
fn number_in_ranges(vec_start: Vec<u64>, vec_end: Vec<u64>) -> u64 {
    let vec_length: usize = vec_start.len();
    let mut starts = vec_start.clone();
    let mut ends = vec_end.clone();

    starts.sort();
    ends.sort();

    let mut total: u64 = 0;

    let mut index: usize = 0;
    while index < vec_length {
        let start = starts[index];
        let mut end = ends[index];

        // println!("starting with {}-{}", start, end);
        while index + 1 < vec_length && starts[index + 1] <= end {
            // println!("{}s < {}e. so it was discarded", starts[index + 1], end);
            if end < ends[index + 1] {
                // println!("{}e < {}e. is new end", end, ends[index + 1]);
                end = ends[index + 1];
            }
            index += 1;
        }
        // print!("from {}-{} got ", start, end);
        let r = food_in_between(start, end);
        // println!("{}", r);
        total += r;
        index += 1;
    }
    return total;
}

fn food_in_between(start: u64, end: u64) -> u64 {
    return end + 1 - start;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut ranges: Vec<String> = Vec::new();
    let mut is_ranges = true;
    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            is_ranges = false;
        }
        if is_ranges {
            ranges.push(l);
        }
    }
    let mut starts: Vec<u64> = vec![];
    let mut ends: Vec<u64> = vec![];
    for range in ranges.clone().into_iter() {
        let mut splits = range.split('-');
        starts.push(
            splits
                .next()
                .expect("first")
                .to_string()
                .parse::<u64>()
                .unwrap(),
        );
        ends.push(
            splits
                .next()
                .expect("second")
                .to_string()
                .parse::<u64>()
                .unwrap(),
        );
    }

    let fresh_foods: u64 = number_in_ranges(starts, ends);
    println!("fresh_foods -> {}", fresh_foods);
    Ok(())
}

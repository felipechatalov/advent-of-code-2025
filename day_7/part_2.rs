use std::fs::File;
use std::io::{BufRead, BufReader};

fn pretty_print(dg: Vec<Vec<char>>) {
    for line in dg {
        for ch in line {
            print!("{}", ch);
        }
        print!("\n");
    }
}

fn traverse_bottom_up(
    dg: &Vec<Vec<char>>,
    line: usize,
    col: usize,
    memo: &mut Vec<Vec<usize>>,
) {
    // print!(
    //     "at line {} col {} max_height {}, max_length {}",
    //     line,
    //     col,
    //     max_height,
    //     dg[0].len()
    // );
    if memo[line][col] != 0 {
        //todo 
        return;
    }
    // if dg[line][col] == '.' {
    //     // no ray passed here
    //     return;
    // }
    if dg[line][col] == '|' {
        // ray couldve went down or he could be splitted
        // in case went down, 'line-1' would be a '|' too
        // 
        memo[line][col] = memo[line+1][col];
        traverse_bottom_up(dg, line-1, col, memo);
        return;
    }
    if dg[line[]]

    if line + 1 >= max_height {
        println!(" end, +1");
        memo[line - 1][col] += 1;
        memo[line][col] += 1;
        return;
    }

    if dg[line + 1][col] == '^' {
        // create left and rigth paths
        println!(" left/right");
        traverse(dg, max_height, line + 1, col - 1, memo);
        traverse(dg, max_height, line + 1, col + 1, memo);
        return;
    }

    //go down
    println!(" down");
    return traverse(dg, max_height, line + 1, col, memo);
}

fn traverse_top_down(
    dg: &Vec<Vec<char>>,
    max_height: usize,
    line: usize,
    col: usize,
    memo: &mut Vec<Vec<usize>>,
) {
    if memo[line][col] != 0 {
        memo[line][col] += 1;
        return;
    }
    print!(
        "at line {} col {} max_height {}, max_length {}",
        line,
        col,
        max_height,
        dg[0].len()
    );
    if line + 1 >= max_height {
        println!(" end, +1");
        memo[line - 1][col] += 1;
        memo[line][col] += 1;
        return;
    }

    if dg[line + 1][col] == '^' {
        // create left and rigth paths
        println!(" left/right");
        traverse(dg, max_height, line + 1, col - 1, memo);
        traverse(dg, max_height, line + 1, col + 1, memo);
        return;
    }

    //go down
    println!(" down");
    return traverse(dg, max_height, line + 1, col, memo);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut diagram: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        diagram.push(line?.chars().collect());
    }
    let height: usize = diagram.len();
    let length: usize = diagram[0].len();

    let mut memo: Vec<Vec<usize>> = vec![vec![0; length]; height];
    for col in 0..length {
        if diagram[height-1][col] == '|' {
            memo[height - 1][col] = 1;
        }
    }
    let mut timeline_count: usize = 0;

    for line in 1..diagram.len() {
        let mut ac: Vec<char> = Vec::new();
        for i in 0..line_length {
            ac.push(diagram[line - 1][i]);
        }
        // println!("dg {:?}", diagram);
        // println!("ac {:?}", ac);

        for (col, ch) in ac.into_iter().enumerate() {
            if ch == 'S' || ch == '|' {
                // laser hiting empty space (continue)
                if diagram[line][col] == '.' {
                    diagram[line][col] = '|';
                }
                // laser hiting a splitter (split to left and rigth)
                if diagram[line][col] == '^' {
                    splitter_count += 1;
                    if col + 1 < line_length {
                        diagram[line][col + 1] = '|'
                    }
                    if col > 0 {
                        diagram[line][col - 1] = '|'
                    }
                }
            }
        }
    }
    for col in 0..length {
        if diagram[height-1][col] == '|' {
            traverse_bottom_up(&diagram, height-1, col, &mut memo);
        }
    }

    // pretty_print(diagram);
    println!("timeline_count -> {}", timeline_count);
    Ok(())
}

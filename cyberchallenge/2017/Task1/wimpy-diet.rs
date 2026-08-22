use std::fs::File;
use std::{io, io::BufReader, io::BufRead};
    
// input
// Line 1: N, number of sandwiches on the menu
// Line 2: N integers

// output
// Line 1: number of sandwiches in the solution
// Line 2: weights of the remaining sandwiches 

// approaching file reading
// map a vector with .window() and check if w[0] is bigger than
// w[1]. if it is than keep both, if it doesnt remove the w[0]
fn main() -> io::Result<()> {
    // 21 input files
    let file = File::open("input/input2.txt")?;
    let file_buffer = BufReader::new(file);

    // to check if to push on n or sandwiches just check the len
    // of the line
    let mut N: u32 = 0;
    let mut sandwiches: Vec<u32> = vec![];

    for line in file_buffer.lines() {
        let line = line?;
        //println!("{}", line);
        // check if this is a single N, if it is then just push everything in a single line
        // check: if the N is empty; if the sanwiches is empty; if line contains whitespaces;  
        if N == 0 {
            N = line.trim().parse().expect("Failed to parse N");
        }
        else {
            let holder: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse().expect("Failed to parse"))
                .collect();
            for k in &holder {
                println!("k: {}", k);
            }
            // Move items
            sandwiches = holder
                .array_windows::<2>()
                // if x > y then do nothing
                // if x < y then return y
                .filter_map(|&[x, y]| {
                    if x < y {
                        Some(y)
                    }
                    else {
                        None
                    }
                })
                .collect();
        }
    }
    println!("{}", sandwiches.len());
    for sandwich in &sandwiches {
        print!("{} ", sandwich)
    }
 
    Ok(())
}

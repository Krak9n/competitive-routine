use std::{fs, io, io::BufRead, path::Path};

fn break_file(filename: &str) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(filename)?;
    // create a new buffered reader
    Ok(io::BufReader::new(file).lines())
}

// break a map into a smaller ones
// start iterating. take the first argument as the size and then
// insert the second line as the iterable, and the third one into a HashMap<u32>
fn main() {
    if let Ok(lines) = break_file("input/subtask1_input1.txt") {
        for line in lines {
            let t: String = line.unwrap();
            let T = t.trim().parse::<u32>().expect("Failed to parse T");

            while T != 0 {
                
            }
        }
    }
}
/*
fn main() -> io::Result<io::Lines<io::BufReader<File>>> {
    io::stdin()
        .read_line(&mut T)
        .expect("Failed to read T.");
    let mut T: u8 = T.trim().parse().expect("Failed to parse T.");
    let mut N: String = String::new();
    let mut result: usize = 0;

    while T != 0 {
        io::stdin()
            .read_line(&mut N)
            .expect("Failed to read N.");
        let N: u8 = N.trim().parse().expect("Failed to parse N.");
        let mut scores: Vec<usize> = read_vec::<usize>();
        if N <= 10 {
            result = scores.iter().sum();
        }
        else if N > 10 {
            result = find_ten_biggest_elements(&mut scores).iter().sum();
        }
        T -= 1;
    }
    println!("{}", result);
}
*/
// Find the biggest one, push it into a new Vector.
// Then remove it from the original. And repeat ten times.
fn find_ten_biggest_elements(inputs: &mut Vec<usize>) -> Vec<usize> {
    let mut mx: Vec<usize> = Vec::new();
    for i in 0..10 {
        let max = inputs.iter().enumerate().max_by_key(|(_, &val)| val);
        match max {
            Some((i, v)) => {
                mx.push(*v);
                inputs.remove(i);
            }
            None => println!("Failed to find the biggest element"),
        }
    }
    mx
}

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

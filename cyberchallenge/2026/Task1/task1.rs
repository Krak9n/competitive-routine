use std::io::BufRead;

fn main() {
    let mut T: String = String::new();
    std::io::stdin()
        .read_line(&mut T)
        .expect("failed to read the T");
    let mut T: u8 = T.trim().parse().expect("failed to parse");
    let mut N: String = String::new();
    let mut result: usize = 0;
    while T != 0 { 
        std::io::stdin()
            .read_line(&mut N)
            .expect("failed to read the N");
        let N: u8 = N.trim().parse().expect("failed to parse");
        let mut scores: Vec<usizE> = read_vec::<usize>();
        if N <= 10 {
            result = scores.iter().sum();
        }
        else if N > 10 {
            result = find_ten_biggest_elements(scores).iter().sum();
        }
        T -= 1;
    }
    println!("{}", result);
}

// find the biggest one
// push it into a new vec
// remove from the original
// repeat ten times
fn find_ten_biggest_elements(mut inputs: Vec<usize>) -> Vec<usize> {
    let mut mx: Vec<usize> = Vec::new();
    for i in 0..10 { 
        let max = inputs
            .iter()
            .enumerate()
            .max_by_key(|(_, &val) | val);
        match max {
            Some((i, v)) => {
                mx.push(*v);
                inputs.remove(i);
            },
            None => println!("failed"),
        }
    }
    mx
}

fn read_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
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

// i have no idea how to implement the reading from file
// from now only from stdin
fn read_lines(filename: &str) -> Vec<String> {
    let t: Vec<String> = std::fs::read_to_string(filename)
        .unwrap()
        .lines() // splits the string into an interator of string slices
        .map(String::from)
        .collect(); // gathers into a vector
    t
}

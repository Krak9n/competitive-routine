use std::io;

fn main() {
    let (mut n, mut whole_line) = (String::new(), String::new());
	let mut numbers: Vec<u64> = Vec::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read n");
    let n: u8 = n.trim().parse().expect("Failed to parse n");
    if n == 2 {
        println!("{}", 2);
        return;
    }
    
    io::stdin()
        .read_line(&mut whole_line)
        .unwrap();
    numbers = whole_line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    numbers
        .sort_by(|a, b| a.cmp(&b));

    for (index, value) in numbers.iter().enumerate() {
        if (index + 1) != *value as usize {
            println!("{}", value - 1);
            break;
        }
    }
}

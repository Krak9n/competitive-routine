use std::io;

// given two positive integers
// x and y
// x is equal my z: x = x / z
// find if x could be equal to y by dividing
fn main() {
    let (mut initial, mut amount) = (String::new(), String::new());
    io::stdin()
        .read_line(&mut amount)
        .expect("failed to read line");
    let mut amount: u32 = s_to_int(amount);
    while amount != 0 {
        io::stdin()
            .read_line(&mut initial)
            .expect("failed to read");
        let n: Vec<u32> = initial
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("failed to parse")) // iterator
            .collect();
        let (x, y): (u32, u32) = (n[0], n[1]); 
        let y = loop {
            
        };
        
        amount -= 1;
    }
}

fn s_to_int(x: String) -> u32 {
    x.trim().parse().expect("Failed to convert String to u32")
}

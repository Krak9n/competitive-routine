use std::fs;

fn main() {
    let file: &str = "./params.txt";
    let insides = read_lines(file);
    let teams: Vec<u32> = insides
        .iter()
        .map(|x| x.parse().expect("failed to convert"))
        .collect();
    println!("team 1: {}", &teams[0]);
}

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .unwrap()
        .lines() // splits the string into an interator of string slices
        .map(String::from)
        .collect(); // gathers into a vector
    contents
}

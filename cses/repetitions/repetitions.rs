use std::io;
use std::collections::HashMap;

// find how many times each of the dna_sequence repeats in the n and output the biggest one
fn main() {
    let dna_sequence = vec!['A', 'C', 'G', 'T'];
        
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Couldn't read the input!");

    let mut freq_map: HashMap<char, i32> = n
        .to_lowercase()  
        .chars()
        .fold(HashMap::new(), |mut map, c| {  // folding a hashamp.
                                              // fold(self, starting value of an accumulator, a closure (current char))
            *map.entry(c).or_insert(0) += 1;  // look up current char in the map  
                                              // and one if it is presnt, insert 0 if it doesnt 
            map  // result of a map
        });

    println!("{}", freq_map
             .iter()
             .max_by(|a, b| a.1.cmp(&b.1)) // compare entries. .1 serves for the (key, value)
             .map(|(key, value)| value)
             .unwrap()); 
}

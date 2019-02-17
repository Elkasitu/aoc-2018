use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;


fn main() {
    println!("Checking input.txt...");

    let mut input = String::new();
    let mut f = File::open("src/input.txt").expect("file not found");
    f.read_to_string(&mut input);

    println!("The output frequency is {}", flatten(input));
}


fn flatten(input: String) -> i32 {
    let mut freq: i32 = 0;
    let mut frequencies: HashSet<i32> = HashSet::new();
    frequencies.insert(0);
    let mut found: bool = false;

    loop {
        let numbers = input.trim().split('\n');
        for number in numbers {
            let number: i32 = number.parse().unwrap();
            freq += number;

            if frequencies.contains(&freq) {
                found = true;
                break;
            } else {
                frequencies.insert(freq);
            }
        }
        if found {
            break;
        }
    }
    return freq;
}

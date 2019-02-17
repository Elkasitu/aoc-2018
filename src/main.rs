use std::fs::File;
use std::io::prelude::*;
use std::str::Split;

fn main() {
    println!("Checking for input.txt...");
    let mut input = String::new();
    let mut f = File::open("src/input.txt").unwrap();
    f.read_to_string(&mut input);
    let barcodes = input.trim().split('\n');

    //println!("The result of the checksum is: {}", calc_checksum(&barcodes));
    println!("The common letters of the correct barcode are: {:?}", get_common_letters(barcodes));
}

fn count(needle: &char, haystack: &str) -> i32 {
    let mut i = 0;
    for c in haystack.chars() {
        if c == *needle {
            i += 1;
        }
    }
    return i;
}

fn has_n(code: &str, n: i32) -> bool {
    for c in code.chars() {
        if count(&c, &code) == n {
            return true;
        }
    }
    return false;
}

fn calc_checksum(barcodes: Split<char>) -> i32 {
    let mut doubles = 0;
    let mut triples = 0;

    for barcode in barcodes {
        if has_n(barcode, 2) {
            doubles += 1;
        }
        if has_n(barcode, 3) {
            triples += 1;
        }
    }

    return doubles * triples;
}

fn cmp_letters(charset: &Vec<char>, charset2: &Vec<char>) -> (bool, Vec<char>) {
    let mut res_bool = false;
    let mut res_vec: Vec<char> = Vec::new();
    let og_size = charset.len() as i32;

    for (x, y) in charset.iter().zip(charset2.iter()) {
        if x == y {
            res_vec.push(*x);
        }
    }

    let res_size = res_vec.len() as i32;
    if og_size == (res_size - 1) {
        res_bool = true;
    }

    return (res_bool, res_vec);
}

fn cmp_words(to_compare: &str, words: Split<char>) -> (bool, Vec<char>) {
    let charset: Vec<char> = to_compare.chars().collect();
    let res_vec: Vec<char> = Vec::new();

    for word in words {
        let charset2: Vec<char> = word.chars().collect();
        let res = cmp_letters(&charset, &charset2);
        if res.0 {
            return res;
        }
    }
    return (false, res_vec);
}

fn get_common_letters(barcodes: Split<char>) -> Vec<char> {
    let res_vec: Vec<char> = Vec::new();
    let barclone = barcodes.clone();
    for barcode in barcodes {
        let res = cmp_words(barcode, barclone.clone());
        if res.0 {
            return res.1;
        }
    }
    return res_vec;
}

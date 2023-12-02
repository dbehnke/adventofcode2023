use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_two_digit_from_code(s: &str) -> i32 {
    // using 99 as a default value
    let mut first_digit = 99;
    let mut last_digit = 99;

    // iterate over the string pulling out the first digit found in string and placing it into first_digit,
    // then pull out the last digit found in string and placing it into last_digit
    for c in s.chars() {
        if c.is_digit(10) {
            if first_digit == 99 {
                first_digit = c.to_digit(10).unwrap() as i32;
                last_digit = first_digit;
            } else {
                last_digit = c.to_digit(10).unwrap() as i32;
            }
        }
    }
    // combine the two digits into a single number
    first_digit * 10 + last_digit
}

fn main() {
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let digits = get_two_digit_from_code(&l);
                sum += digits;
            }
        }
        println!("Sum of all two digit numbers in input.txt is {}", sum);
    }
}

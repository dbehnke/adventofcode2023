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

fn get_two_digit_from_code_with_words(s: &str) -> i32 {
    //replace all words with numbers

    /*
    o n   e eight
    t w   o one
    t hre e eight
    f ou  r 
    f iv  e eight
    s i   x 
    s eve n nine
    e igh t two
    n inn e eight

    
     */

    let mut s = s.replace("oneight", "oneeight");
    s = s.replace("twone", "twoone");
    s = s.replace("threeight", "threeeight");
    s = s.replace("fiveight", "fiveeight");
    s = s.replace("sevenine", "sevennine");
    s = s.replace("eightwo", "eighttwo");
    s = s.replace("eighthree", "eightthree");
    s = s.replace("nineight", "nineeight");
    s = s.replace("one", "1");
    s = s.replace("two", "2");
    s = s.replace("three", "3");
    s = s.replace("four", "4");
    s = s.replace("five", "5");
    s = s.replace("six", "6");
    s = s.replace("seven", "7");
    s = s.replace("eight", "8");
    s = s.replace("nine", "9");
    //println!("s: {} answer: {}", s, get_two_digit_from_code(s.as_str()));

    return get_two_digit_from_code(s.as_str());
}

fn part_one() -> i32 {
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
        return sum;
    }
    0
}

fn part_two() -> i32 {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(l) = line {
                let digits = get_two_digit_from_code_with_words(&l);
                sum += digits;
            }
        }
        return sum;
    }
    0
}

fn main() {
    println!("The answer for part one is: {}", part_one());
    println!("The answer for part two is: {}", part_two());
}

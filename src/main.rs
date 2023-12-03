use regex::Regex;
use std::env;
use std::fs;

fn num_string_to_int(num_string: &str) -> i32 {
    match num_string {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        &_ => 0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("filename: {}", filename);

    let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut sum = 0;

    let regex = Regex::new(r"([0-9])").unwrap();

    contents = contents
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        ;


    for line in contents.lines() {
        let mut first_match = String::new();
        let mut last_match = String::new();

        for cap in regex.find_iter(line) {
            if first_match.is_empty() {
                first_match = cap.as_str().to_string();
            }
            last_match = cap.as_str().to_string();
        }

        let first_num = num_string_to_int(first_match.as_str());
        let last_num = num_string_to_int(last_match.as_str());

//        println!("{}, first_num: {}, last_num: {}", line, first_num, last_num);

        sum +=
            num_string_to_int(first_match.as_str()) * 10 + num_string_to_int(last_match.as_str());
    }

    println!("{}", sum);
}

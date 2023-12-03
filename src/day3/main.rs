use regex::Captures;
use regex::Regex;
use std::env;
use std::fs;
use std::str;

/*
fn is_line_match(
    line: &str,
    prev_line: &str,
    next_line: &str,
    start_index: usize,
    end_index: usize,
    max_index: usize,
) -> bool {
    // println!("{:?} {:?} {:?}", line, prev_line, next_line);
    let mut start = 0;
    let mut end = 0;
    if start_index > 0 {
        start = start_index - 1;
    } else {
        start = start_index;
    }
    if end_index < max_index {
        end = end_index + 1;
    } else {
        end = end_index;
    }
    for i in start..end {
        if prev_line[i] != '.' as u8 || next_line[i] != '.' as u8 {
            return true;
        }
        if (i == start || i == end) && line[i] != '.' as u8 {
            return true;
        }
    }
    return false;
}
*/

fn is_line_match(
    line: &str,
    prev_line: &str,
    next_line: &str,
    capture: &Captures,
    max_index: usize,
) -> bool {
    let mut start = capture.get(0).unwrap().start();
    let mut end = capture.get(0).unwrap().end();
    let prev_line_bytes = prev_line.as_bytes();
    let next_line_bytes = next_line.as_bytes();
    let current_line_bytes = line.as_bytes();
    if start > 0 {
        start -= 1;
    }
    if end < max_index {
        end += 1;
    }

    for i in start..end {
        if prev_line_bytes[i] != '.' as u8
            && prev_line_bytes[i] < '0' as u8
            && prev_line_bytes[i] > '9' as u8
        {
            return true;
        }
        if next_line_bytes[i] != '.' as u8
            && next_line_bytes[i] < '0' as u8
            && next_line_bytes[i] > '9' as u8
        {
            return true;
        }

        if i == start || i == end {
            if current_line_bytes[i] != '.' as u8
                && current_line_bytes[i] < '0' as u8
                && current_line_bytes[i] > '9' as u8
            {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file = &args[1];

    let mut contents = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = contents.trim().split("\n").collect();

    let dummy_string = ".".repeat(lines[0].len());

    let numbers_regex = Regex::new(r"\d+").unwrap();

    for i in 0..lines.len() {
        let line = &lines[i];
        let mut prev_line = dummy_string.as_str();
        let mut next_line = dummy_string.as_str();
        if i == 0 {
            prev_line = &dummy_string;
        } else {
            prev_line = &lines[i - 1];
        }
        if i == lines.len() - 1 {
            next_line = &dummy_string;
        } else {
            next_line = &lines[i + 1];
        }

        for capture in numbers_regex.captures_iter(line) {
            let is_match = is_line_match(line, prev_line, next_line, &capture, lines[0].len());
            println!("{}", is_match);
            if is_match {
                println!("{:?}", capture.get(0).unwrap().start());
            }
        }
    }
}

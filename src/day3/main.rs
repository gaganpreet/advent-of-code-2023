use regex::bytes::Regex;
use std::env;
use std::fs;
use std::str;
use std::str::from_utf8;

fn line_meets_criteria(
    line: &&[u8],
    prev_line: &[u8],
    next_line: &[u8],
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file = &args[1];

    let mut contents = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = contents.trim().split("\n").collect();

    let bytelines = lines.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();

    let line_len = lines[0].len();

    // make a string of length with just ignored character '.'
    let dummy_string = ".".repeat(line_len);
    let dummy_string_bytes = dummy_string.as_bytes();
    let mut next_line = bytelines[1];
    let mut prev_line = dummy_string_bytes;
    let mut sum = 0;

    let find_numbers_regex = Regex::new(r"(\d+)").unwrap();

    for (i, line) in bytelines.iter().enumerate() {
        // println!("##{} {}", i, bytelines.len());
        if (i + 1) == bytelines.len() {
            next_line = dummy_string_bytes;
        } else {
            next_line = bytelines[i + 1];
        }

        for cap in find_numbers_regex.captures_iter(line) {
            let start = cap.get(1).unwrap().start();
            let end = cap.get(1).unwrap().end();

            if (line_meets_criteria(line, prev_line, next_line, start, end, line_len)) {
                let parsed_num_str = cap.get(0).unwrap().as_bytes();

                let num = from_utf8(parsed_num_str).unwrap().parse::<i32>().unwrap();
                sum += num;
                println!(">>{:?}", num);
                //println!("{}", sum);
            }
            //println!("start: {}, end: {}, line: {:?}", start, end, line);
        }
        //println!("---");

        //println!("{}\n{}\n{}\n", prev_line, line, next_line);
        //println!("----");

        prev_line = line;
    }
    println!("Sum: {}", sum);
}

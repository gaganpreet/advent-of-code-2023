use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

fn string_to_int_set(string: &str) -> HashSet<u32> {
    let mut res: HashSet<u32> = HashSet::new();
    let numbers = string
        .trim()
        .split(" ")
        .filter(|x| x.trim().len() > 0)
        .map(|x| x.trim().parse::<u32>().unwrap());

    for number in numbers {
        res.insert(number);
    }
    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file = &args[1];

    let contents = fs::read_to_string(file).unwrap();

    let regex = Regex::new(r"Card *(\d+): (.*) \| (.*)").unwrap();
    let mut points = 0;
    let power_base: i32 = 2;

    for line in contents.lines() {
        let captures = regex.captures(line).unwrap();
        let game = captures.get(1).unwrap().as_str();
        let winning_numbers_str = captures.get(2).unwrap().as_str();
        let have_numbers_str = captures.get(3).unwrap().as_str();

        let winning_numbers_set = string_to_int_set(winning_numbers_str);
        let have_numbers_set = string_to_int_set(have_numbers_str);

        let have_count = have_numbers_set.intersection(&winning_numbers_set).count();

        /*
        println!(
            "Card {}: {:?} | {:?}: {:?} {}",
            game,
            winning_numbers_set,
            have_numbers_set,
            have_numbers_set.intersection(&winning_numbers_set),
            have_count,
        );
        */
        if have_count > 0 {
            points += power_base.pow((have_count - 1) as u32);
        }
    }
    println!("Points: {}", points);
}

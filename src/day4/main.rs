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
    let mut points_part_2 = 0;
    let power_base: i32 = 2;

    let mut extra_card_vector: Vec<u32> = Vec::with_capacity(contents.split("\n").count() - 1);

    for i in 0..contents.split("\n").count() - 1 {
        extra_card_vector.push(0);
    }

    for (i, line) in contents.lines().enumerate() {
        let captures = regex.captures(line).unwrap();
        let game = captures.get(1).unwrap().as_str();
        let mut extra_points = 0;
        let winning_numbers_str = captures.get(2).unwrap().as_str();
        let have_numbers_str = captures.get(3).unwrap().as_str();

        let winning_numbers_set = string_to_int_set(winning_numbers_str);
        let have_numbers_set = string_to_int_set(have_numbers_str);

        let have_count = have_numbers_set.intersection(&winning_numbers_set).count();
        // println!("{:?}", extra_card_vector);

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
        for j in i + 1..i + 1 + have_count {
            extra_card_vector[j] += 1 + extra_card_vector[i];
        }

        // println!("{}", extra_card_vector[i] + have_count as u32);

        if have_count > 0 {
            points += power_base.pow((have_count - 1) as u32);
        }
        points_part_2 += extra_card_vector[i] + 1;
    }
    println!("Points: {} {}", points, points_part_2);
}

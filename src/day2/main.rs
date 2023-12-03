use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file = &args[1];

    let contents = fs::read_to_string(file).unwrap();

    let line_regex = Regex::new(r"Game ([0-9]+): (.*)").unwrap();

    let mut sum = 0;
    let mut sum_power_cubes = 0;

    for line in contents.lines() {
        let captures = line_regex.captures(line).unwrap();

        let game = captures.get(1).unwrap().as_str();
        let game_num = game.parse::<i32>().unwrap();
        let sets = captures.get(2).unwrap().as_str();

        let mut min_blue = 0;
        let mut min_red = 0;
        let mut min_green = 0;

        let mut is_possible = true;

        for set in sets.split(";") {
            for draw in set.split(",") {
                let num_and_colour = draw.split_whitespace().collect::<Vec<&str>>();
                let num = num_and_colour[0].parse::<i32>().unwrap();
                let colour = num_and_colour[1];

                if colour == "red" {
                    if num > min_red {
                        min_red = num;
                    }
                    if num > 12 {
                        is_possible = false;
                    }
                } else if colour == "green" {
                    if num > min_green {
                        min_green = num;
                    }
                    if num > 13 {
                        is_possible = false;
                    }
                } else if colour == "blue" {
                    if num > min_blue {
                        min_blue = num;
                    }
                    if num > 14 {
                        is_possible = false;
                    }
                }
            }
        }
        let power_cube = min_red * min_blue * min_green;
        //println!(
        //"game: {}, line: {}, is_possible: {}, min_red: {}, min_green: {}, min_blue: {}, power_cube: {}",
        //game_num, line, is_possible, min_red, min_green, min_blue, power_cube
        //);
        if is_possible {
            sum += game_num;
        }
        sum_power_cubes += power_cube;
    }

    println!("{} {}", sum, sum_power_cubes);
}

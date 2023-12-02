use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input1").expect("Something went wrong reading the file");

    let mut sum = 0;

    // Note: regex is greedy, so it will match as many digits as it can
    // We are just looking for the first and last digits
    let regex = Regex::new(r"^.*([0-9]).*").unwrap();

    for line in contents.lines() {

        let num1 = regex.captures(line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();

        let reversed_line = line.chars().rev().collect::<String>();

        let num2 = regex.captures(&reversed_line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();

//        println!("{} {} {}", line, num1, num2);

        sum += num2 * 10 + num1;
    }




    /*
       // old brute force impl
    for line in contents.lines() {
        // There are multiple digits in each line amongst letters, we need only the first and the last one to make a two digit number
        let mut num = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                num = 10 * c.to_digit(10).unwrap();
                break;
            }
        }

        // now iterate in reverse order to get the last digit
        for c in line.chars().rev() {
            if c.is_digit(10) {
                num = num + c.to_digit(10).unwrap();
                break;
            }
        }

        println!("{} {}", line, num);

        sum += num;
    }
    */
    println!("{}", sum);
}

use std::fs;
use std::io::Error;

mod day1;

fn main() {
    let file: Result<String, Error> = fs::read_to_string("src/data/day1_sample.txt");
    let mut left_side: Vec<i32> = vec![];
    let mut right_side: Vec<i32> = vec![];
    match file {
        Ok(contents) => {
            for line in contents.split('\n') {
                let split: Vec<&str> = line.split(' ').collect();
                left_side.push(split[0].parse::<i32>().unwrap());
                right_side.push(split[split.len() - 1].parse::<i32>().unwrap());
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    left_side.sort();
    right_side.sort();

    let mut sum: i32 = 0;

    for index in 0..left_side.len() {
        let distance = (left_side[index] - right_side[index]).abs();
        sum += distance;
    }
    println!("{}", sum);
}

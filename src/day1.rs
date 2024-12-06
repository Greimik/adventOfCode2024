use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::ops::AddAssign;

pub fn solve_day1() -> i32 {
    let file: Result<String, Error> = fs::read_to_string("src/data/day1_sample.txt");
    let mut left_side: Vec<i32> = vec![];
    let mut right_side: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    let mut key_by_occurence: HashMap<i32, i32> = HashMap::new();
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


    for key in left_side.iter() {
        if !key_by_occurence.contains_key(key) {
            key_by_occurence.insert(*key, 0);
            for right_item in right_side.iter() {
                if right_item == key {
                    key_by_occurence.get_mut(key).unwrap().add_assign(1);
                }
            }
        }
    }

    for key in left_side.iter() {
        let similarity = *key * key_by_occurence.get(key).unwrap();
        sum += similarity;
    }
    return sum;
}
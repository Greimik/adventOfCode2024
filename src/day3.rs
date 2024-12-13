use regex::Regex;

pub fn solve_day3() -> i32 {
    let regex: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let file_input = std::fs::read_to_string("src/data/day3_input.txt").unwrap();
    let finds: Vec<&str> = regex.find_iter(&file_input).map(|x| x.as_str()).collect();
    let mut sum: i32 = 0;
    for find in finds {
        let items : Vec<i32> = find.replace("mul(", "").replace(")", "")
            .split(',')
            .map(|x| x.parse::<i32>().unwrap()).collect();
        let multiply = items[0]*items[1];
        sum += multiply;
    }
    return sum;
}
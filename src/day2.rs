use std::fs;
use std::io::Error;
const MIN_DIFFERENCE: i32 = 1;
const MAX_DIFFERENCE: i32 = 3;

pub fn solve_day2() -> i32 {
    let input_data: Result<String, Error> = fs::read_to_string("src/data/day2_input.txt");
    let mut safe_report_count: i32 = 0;

    match input_data {
        Ok(data) => {
            for level in data.split('\n') {
                let mut chunks: Vec<i32> = level.split(' ')
                    .map(|s| s.parse().unwrap())
                    .collect();

                let (is_safe_full, failing_chunk_full) = check_line(&chunks);
                let (is_safe_partial, failing_chunk_partial) = check_line(&chunks[1..]);
                if is_safe_partial && is_safe_full {
                    safe_report_count += 1;
                } else {
                    println!("level {} full failing {} parial failing {}", level, failing_chunk_full, failing_chunk_partial);
                }
            }
            println!("Safe report count: {}", safe_report_count);
        }
        Err(error) => {
            println!("Failed to read input: {}", error);
        }
    }

    return safe_report_count;
}

fn check_line(chunks: &[i32]) -> (bool, usize) {
    let is_increasing = chunks[1] > chunks[0];
    let mut safe_levels: Vec<i32> = Vec::new();
    safe_levels.push(chunks[0]);
    let mut failing_chunk_index: usize = 0;

    for index in 0..chunks.len() - 1 {
        let difference = (chunks[index] - chunks[index + 1]).abs();
        if difference < MIN_DIFFERENCE || difference > MAX_DIFFERENCE {
            failing_chunk_index = index + 1;
            break;
        }

        if is_increasing && chunks[index] >= chunks[index + 1] {
            failing_chunk_index = index + 1;
            break;
        }

        if !is_increasing && chunks[index] <= chunks[index + 1] {
            failing_chunk_index = index + 1;
            break;
        }

        safe_levels.push(chunks[index + 1]);
    }

    (safe_levels.len() == chunks.len(), failing_chunk_index)
}


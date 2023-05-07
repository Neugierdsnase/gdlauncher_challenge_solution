use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

const RELEVANT_NUMBERS: usize = 100;

fn has_sum_pair(numbers: &VecDeque<usize>, target: usize) -> (usize, bool) {
    let mut num_set = HashSet::new();

    for &num in numbers.iter() {
        if num_set.contains(&target.wrapping_sub(num)) {
            return (target, true);
        }
        num_set.insert(num);
    }

    (target, false)
}

fn parsed_number_from_read_line(line_result: Result<String, Error>) -> usize {
    let num = match line_result {
        Ok(line) => line.parse::<usize>().unwrap(),
        // Not nice, should be handled.
        Err(e) => {
            panic!("Aaaaah! {}", e)
        }
    };

    num
}

fn main() {
    let path = Path::new("./input_sample.txt");
    // Not nice, should also be handled.
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let cave = reader.lines();

    let mut relevant_numbers_deque: VecDeque<usize> = VecDeque::with_capacity(RELEVANT_NUMBERS);

    for (index, line_result) in cave.enumerate() {
        // Filling up the deque first
        if index < RELEVANT_NUMBERS {
            let num = parsed_number_from_read_line(line_result);
            relevant_numbers_deque.push_back(num);
            continue;
        };

        let num = parsed_number_from_read_line(line_result);
        let (result, is_safe) = has_sum_pair(&relevant_numbers_deque, num);
        if !is_safe {
            println!("{:?} is unsafe.", result);
            return;
        }
        relevant_numbers_deque.pop_front();
        relevant_numbers_deque.push_back(num);
    }
}

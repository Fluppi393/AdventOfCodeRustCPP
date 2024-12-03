use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn main(reader: BufReader<File>) -> (String, String) {
    // Build the vector
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        // Find the end of the left number and start of the right one
        let left_end = line.find(' ').unwrap();
        let right_start = line.rfind(' ').unwrap() + 1;

        let left_str = &line[0..left_end];
        let right_str = &line[right_start..line.len()];

        let left_num = left_str.parse::<i32>().unwrap();
        let right_num = right_str.parse::<i32>().unwrap();

        left_numbers.push(left_num);
        right_numbers.push(right_num);
    }

    // Sort so we can add side by side
    left_numbers.sort();
    right_numbers.sort();

    // Calculate the result
    let mut result = 0;
    let mut similarity = 0;
    for i in 0..left_numbers.len() {
        let left_num = left_numbers[i];
        let right_num = right_numbers[i];
        result += (left_num - right_num).abs();

        let num = right_numbers.iter().filter(|&x| *x == left_num).count() as i32;
        similarity += left_num * num;
    }

    (result.to_string(), similarity.to_string())
}

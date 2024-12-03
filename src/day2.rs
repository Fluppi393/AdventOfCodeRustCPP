use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn main(reader: BufReader<File>) -> (String, String) {
    fn is_safe_report(report: &Vec<i32>) -> bool {
        let mut is_increasing = false;
        for i in 1..report.len() {
            let lvl = report[i];
            let prev_lvl = report[i - 1];
            let delta = (lvl - prev_lvl).abs();

            // Make sure the delta between levels isn't too big
            if delta < 1 || delta > 3 {
                return false;
            }

            // Make sure all levels are either increasing or decreasing
            let new_increasing = lvl > prev_lvl;
            if i > 1 && new_increasing != is_increasing {
                return false;
            }
            is_increasing = new_increasing;
        }

        true
    }

    let mut num_safe = 0;
    let mut num_safe_2 = 0;
    let mut report = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        // Build the report
        report.clear();
        for token in line.split_whitespace() {
            let lvl = token.parse::<i32>().unwrap();
            report.push(lvl);
        }

        if is_safe_report(&report) {
            num_safe += 1;
            num_safe_2 += 1;
        }
        else {
            // Test if removing any element would make the report safe
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);

                if is_safe_report(&new_report) {
                    num_safe_2 += 1;
                    break;
                }
            }
        }
    }

    (num_safe.to_string(), num_safe_2.to_string())
}
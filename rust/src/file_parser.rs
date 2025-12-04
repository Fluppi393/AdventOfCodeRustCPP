use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn find_input_dir() -> Option<PathBuf> {
    let mut dir = env::current_dir().ok().unwrap();

    loop {
        let input_dir = dir.join("input");
        if input_dir.is_dir() {
            return Some(input_dir);
        }

        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => break,
        }
    }

    None
}

pub struct FileParser {
    lines: std::io::Lines<BufReader<File>>,
    cur_line: String,
    char_idx: usize,
}

impl FileParser {
    pub fn new(filename: &str) -> FileParser {
        let file_path = find_input_dir().unwrap().join(Path::new(filename));
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();
        let cur_line = "".to_string();

        FileParser {
            lines,
            cur_line,
            char_idx: 0,
        }
    }

    pub fn inline(input: &str) -> FileParser {
        // Create a temporary file
        let filename = "temp.txt";
        let mut temp_file = File::create(filename).unwrap();
        // Write test data to the temporary file
        temp_file.write_all(input.as_bytes()).unwrap();
        FileParser::new(filename)
    }

    pub fn next_line(&mut self) -> bool {
        match self.lines.next() {
            Some(line) => {
                self.cur_line = line.unwrap();
                self.char_idx = 0;
                !self.is_line_done()
            }
            None => false,
        }
    }

    pub fn get_line(&self) -> &str {
        &self.cur_line
    }

    pub fn next_char(&mut self) -> bool {
        self.char_idx = self.char_idx + 1;
        !self.is_line_done()
    }

    pub fn next_char_or_line(&mut self) -> bool {
        if !self.next_char() {
            self.next_line()
        } else {
            true
        }
    }

    pub fn is_line_done(&self) -> bool {
        self.char_idx >= self.cur_line.len()
    }

    pub fn skip(&mut self) {
        self.char_idx += 1;
    }

    pub fn get_char(&self, offset: usize) -> Option<char> {
        self.cur_line.chars().nth(self.char_idx + offset)
    }

    pub fn is_char(&self, c: char) -> bool {
        self.get_next_char() == Some(c)
    }

    pub fn get_next_char(&self) -> Option<char> {
        self.get_char(0)
    }

    pub fn skip_num(&mut self, num: usize) -> bool {
        if num > 0 && self.char_idx + num <= self.cur_line.len() {
            self.char_idx += num;
            return true;
        }

        false
    }

    pub fn skip_to_line_end(&mut self) -> bool {
        self.skip_num(self.cur_line.len() - self.char_idx)
    }

    pub fn consume_num(&mut self, num: usize) -> Option<String> {
        if self.skip_num(num) {
            return Some(self.cur_line[self.char_idx - num..self.char_idx].to_string());
        }

        None
    }

    pub fn count_by_predicate<F>(&self, predicate: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut num = 0;
        while self.get_char(num).map_or(false, |c| predicate(c)) {
            num += 1;
        }

        num
    }

    // Advances over all characters that match a certain predicate
    pub fn skip_by_predicate<F>(&mut self, predicate: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let num = self.count_by_predicate(predicate);
        self.char_idx += num;
        num
    }

    pub fn skip_whitespace(&mut self) -> usize {
        self.skip_by_predicate(|c| c.is_whitespace())
    }

    // Returns a string of all adjacent characters that match a certain predicate
    pub fn consume_by_predicate<F>(&mut self, predicate: F) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        self.consume_num(self.count_by_predicate(predicate))
    }

    pub fn consume_any_char(&mut self) -> Option<char> {
        let result = self.get_next_char();
        if result.is_some() {
            self.skip();
        }

        result
    }

    pub fn consume_char(&mut self, c: char) -> bool {
        if self.is_char(c) {
            self.skip();
            return true;
        }

        false
    }

    pub fn consume_digit(&self) -> Option<u32> {
        self.get_next_char().and_then(|c| c.to_digit(10))
    }

    pub fn is_string(&self, to_find: &str) -> bool {
        let mut num = 0;
        while num < to_find.len() && self.get_char(num) == to_find.chars().nth(num) {
            num += 1;
        }

        num == to_find.len()
    }

    pub fn consume_string(&mut self, to_find: &str) -> bool {
        self.skip_num(if self.is_string(to_find) {
            to_find.len()
        } else {
            0
        })
    }

    pub fn count_numeric(&self) -> usize {
        self.count_by_predicate(|c| c.is_numeric())
    }

    pub fn consume_numeric<T>(&mut self) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.consume_num(self.count_numeric())
            .map(|s| s.parse::<T>().unwrap())
    }

    pub fn consume_i32(&mut self) -> Option<i32> {
        self.consume_numeric()
    }

    pub fn count_non_numeric(&self) -> usize {
        self.count_by_predicate(|c| !c.is_numeric())
    }

    pub fn count_alphabetic(&self) -> usize {
        self.count_by_predicate(|c| c.is_alphabetic())
    }

    pub fn consume_alphabetic(&mut self) -> Option<String> {
        self.consume_num(self.count_alphabetic())
    }

    pub fn count_alphanumeric(&self) -> usize {
        self.count_by_predicate(|c| c.is_alphanumeric())
    }

    pub fn consume_alphanumeric(&mut self) -> Option<String> {
        self.consume_num(self.count_alphanumeric())
    }

    pub fn count_whitespace(&self) -> usize {
        self.count_by_predicate(|c| c.is_whitespace())
    }

    pub fn consume_whitespace(&mut self) -> Option<String> {
        self.consume_num(self.count_whitespace())
    }
}

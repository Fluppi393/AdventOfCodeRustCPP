use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct StringParser {
    string: String,
    index: usize
}

impl StringParser {
    pub fn new(string: &str) -> StringParser {
        StringParser {
            string: string.to_string(),
            index: 0
        }
    }

    pub fn is_done(&self) -> bool {
        self.index >= self.string.len()
    }

    pub fn skip(&mut self) {
        self.index += 1;
    }

    pub fn get_char(&self, offset: usize) -> Option<char> {
        self.string.chars().nth(self.index + offset)
    }

    pub fn is_char(&self, c: char) -> bool {
        self.get_next_char() == Some(c)
    }

    pub fn get_next_char(&self) -> Option<char> {
        self.get_char(0)
    }

    pub fn skip_num(&mut self, num: usize) -> bool
    {
        if num > 0 && self.index + num <= self.string.len() {
            self.index += num;
            return true;
        }

        false
    }

    pub fn consume_num(&mut self, num: usize) -> Option<String>
    {
        if self.skip_num(num) {
            return Some(self.string[self.index - num..self.index].to_string());
        }

        None
    }

    pub fn count_by_predicate<F>(&self, predicate: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut num = 0;
        while self.get_next_char().map(|c| predicate(c)).is_some() {
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
        self.index += num;
        num
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

    pub fn is_string(&self, to_find: &str) -> bool {
        let mut num = 0;
        while num < to_find.len() && self.get_char(num) == to_find.chars().nth(num) {
            num += 1;
        }

        num == to_find.len()
    }

    pub fn consume_string(&mut self, to_find: &str) -> bool {
        self.skip_num(if self.is_string(to_find) { to_find.len() } else { 0 })
    }


    pub fn count_numeric(&self) -> usize {
        self.count_by_predicate(|c| c.is_numeric())
    }
    pub fn consume_i32(&mut self) -> Option<i32> {
         self.consume_num(self.count_numeric()).map(|s| s.parse::<i32>().unwrap())
    }
}

pub struct FileParser {
    lines: std::io::Lines<BufReader<File>>
}

impl FileParser {
    pub fn new(filename: &str) -> FileParser {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();
        FileParser {
            lines
        }
    }

    pub fn parse_line(&mut self) -> Option<StringParser> {
        self.lines.next().map(|line| StringParser::new(&line.unwrap()))
    }
}
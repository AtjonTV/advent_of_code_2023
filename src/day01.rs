use crate::util::{assert, read_file_by_lines};
use crate::Challenge;
use pcre2::bytes::{Captures, Regex};
use std::io;
use std::path::PathBuf;

pub struct Day01(String);
impl Day01 {
    fn str_to_num(&self, input: &str) -> u32 {
        match input {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => unreachable!(),
        }
    }
}

impl Challenge for Day01 {
    fn new() -> Self {
        Day01("d01".to_string())
    }

    fn id(&self) -> &String {
        &self.0
    }

    fn run(&self, challenge: &str, file: &PathBuf) -> io::Result<bool> {
        match challenge {
            "p1" => {
                let lines = read_file_by_lines(file)?;
                let mut sum = 0u32;
                for line in lines {
                    let nums = line
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<Vec<char>>();
                    let first = *nums.first().unwrap();
                    let last = *nums.last().unwrap();
                    let num = format!("{}{}", first, last);
                    sum += num.parse::<u32>().unwrap();
                }
                println!("Result of Day 1 Part 1: {}", sum);
                assert(54632u32, sum);
            }
            "p2" => {
                let lines = read_file_by_lines(file)?;
                let rex = Regex::new(r"(?=([1-9]|one|two|three|four|five|six|seven|eight|nine))")
                    .unwrap();

                let mut sum = 0u32;
                for line in lines {
                    let matches = rex
                        .captures_iter(line.as_str().as_bytes())
                        .collect::<Vec<Result<Captures, pcre2::Error>>>();
                    if matches.is_empty() {
                        continue;
                    }
                    let first_match = matches.iter().nth(0).unwrap().as_ref().unwrap();
                    let last_match = matches
                        .iter()
                        .nth(matches.len() - 1)
                        .unwrap()
                        .as_ref()
                        .unwrap();

                    let first_bytes = first_match.get(1).unwrap().as_bytes();
                    let last_bytes = last_match.get(1).unwrap().as_bytes();

                    let first = String::from_utf8_lossy(first_bytes).to_string();
                    let last = String::from_utf8_lossy(last_bytes).to_string();

                    let num = format!(
                        "{}{}",
                        self.str_to_num(first.as_str()),
                        self.str_to_num(last.as_str())
                    );
                    sum += num.parse::<u32>().unwrap();
                }
                println!("Result of Day 1 Part 2: {}", sum);
                assert(54019u32, sum);
            }
            _ => unreachable!(),
        }
        Ok(false)
    }
}

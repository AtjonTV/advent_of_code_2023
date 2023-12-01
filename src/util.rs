use crate::Challenge;
use std::fmt::Debug;
use std::path::PathBuf;
use std::{fs, io};

pub fn read_file_by_lines(path: &PathBuf) -> io::Result<Vec<String>> {
    let input = fs::read_to_string(path.clone())?;

    Ok(input
        .lines()
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect())
}

pub fn assert<T: PartialEq + Debug>(expect: T, actual: T) {
    assert_eq!(expect, actual);
}

pub fn boxed<T: Challenge>(val: T) -> Box<T> {
    Box::new(val)
}

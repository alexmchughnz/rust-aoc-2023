use std::{env, fs};

pub fn read_input() -> String {
    let input_path = env::current_dir().unwrap().join("input");
    let input = fs::read_to_string(input_path).expect("should read input file");
    input
}

pub fn get_day_num(path: &str) -> String {
    path.split('/')
        .find(|seg| seg.parse::<u32>().is_ok())
        .expect("a numeric path segment should exist")
        .to_owned()
}

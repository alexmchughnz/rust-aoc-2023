use std::{env, fs};

pub fn read_input(path: &str, filename: &str) -> String {
    let mut input_path = path.trim_end_matches(|c| c != '/').to_string();
    input_path += filename;
    let input = fs::read_to_string(input_path).expect("should read specified input file");
    input
}

pub fn get_day_num(path: &str) -> String {
    path.split('/')
        .find(|seg| seg.parse::<u32>().is_ok())
        .expect("a numeric path segment should exist")
        .to_owned()
}

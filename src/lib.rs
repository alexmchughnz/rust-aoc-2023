pub mod helpers;

use std::fs;

pub fn read_input(path: &str, filename: &str) -> String {
    let mut input_path = path.trim_end_matches(|c| c != '/').to_string();
    input_path += filename;
    let input = fs::read_to_string(&input_path)
        .expect(format!("should read specified input file ({})", &input_path).as_str());
    input
}

pub fn get_day_str(path: &str) -> String {
    path.split('/')
        .find(|seg| seg.contains("day"))
        .expect("a /day[x]/ path segment should exist")
        .to_owned()
}

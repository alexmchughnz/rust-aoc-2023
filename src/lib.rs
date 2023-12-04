use std::{env, fs};

pub fn read_input() -> String {
    let input_path = env::current_dir().unwrap().join("input");
    let input = fs::read_to_string(input_path).expect("should read input file");
    input
}

pub fn get_day_num() -> String {
    let day_dir = env::current_dir().unwrap();
    let day_str = day_dir.into_iter().last().unwrap().to_str().unwrap();
    let day_num = day_str
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    day_num
}

pub mod helpers;

use std::{fs, path::Path};

pub fn read_input(path: &str, filename: &str) -> String {
    let path = path.trim_end_matches(|c| c != '/'); // Removes trailing "main.rs"
    let abs_input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(path)
        .join(filename);

    let input = fs::read_to_string(&abs_input_path).expect(
        format!(
            "should read specified input file ({})",
            abs_input_path.display()
        )
        .as_str(),
    );
    input
}

pub fn get_day_str(path: &str) -> String {
    path.split('/')
        .find(|seg| seg.contains("day"))
        .expect("a /day[x]/ path segment should exist")
        .to_owned()
}

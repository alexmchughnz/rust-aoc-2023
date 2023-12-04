use aoc2023::{get_day_num, read_input};

fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let mut digits = String::with_capacity(2);
        digits.push(line.chars().find(|c| c.is_ascii_digit()).unwrap());
        digits.push(line.chars().rfind(|c| c.is_ascii_digit()).unwrap());

        total += digits
            .parse::<u32>()
            .expect("should parse valid 2-digit number");
    }

    Some(total)
}

fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = read_input(file!(), "input");
    let day = get_day_num(file!());
    println!("{}", file!());
    println!("Day {day}.1 Solution: {:?}", part_one(&input));
    println!("Day {day}.2 Solution: {:?}", part_two(&input));
}

use aoc2023::{get_day_num, read_input};

struct Digit {
    value: u32,
    index: usize,
}

const DIGIT_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digit(s: &str) -> u32 {
    match s.parse::<u32>() {
        Ok(v) => v, // e.g. "1"
        Err(_) => {
            // e.g. "one"
            let index = DIGIT_NAMES
                .iter()
                .position(|name| s == *name)
                .expect("must be valid digit name") as u32;
            index + 1
        }
    }
}

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

fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for line in input.lines() {
        // Find outermost digit chars.
        let first = line.chars().enumerate().find(|tup| tup.1.is_ascii_digit());
        let mut leftmost: Digit;
        if let Some((index, ch)) = first {
            leftmost = Digit {
                value: parse_digit(&ch.to_string()),
                index,
            }
        } else {
            leftmost = Digit {
                value: u32::MAX,
                index: usize::MAX,
            }
        }

        let last = line
            .chars()
            .rev()
            .enumerate()
            .find(|tup| tup.1.is_ascii_digit());
        let mut rightmost: Digit;
        if let Some((index, ch)) = last {
            rightmost = Digit {
                value: parse_digit(&ch.to_string()),
                index: line.len() - 1 - index,
            }
        } else {
            rightmost = Digit {
                value: u32::MAX,
                index: 0,
            }
        }

        // Search for even-more-outer digit names.
        DIGIT_NAMES.iter().for_each(|substr| {
            // Update leftmost "digit".
            if let Some(index) = line.find(substr) {
                if index < leftmost.index {
                    leftmost = Digit {
                        value: parse_digit(substr),
                        index,
                    }
                }
            }

            // Update rightmost "digit".
            if let Some(index) = line.rfind(substr) {
                if index > rightmost.index {
                    rightmost = Digit {
                        value: parse_digit(substr),
                        index,
                    }
                }
            }
        });

        // Sum outermost "digits". e.g. "one" and "2" makes 12.
        total += 10 * leftmost.value + rightmost.value
    }

    Some(total)
}

fn main() {
    let input = read_input(file!(), "input");
    let day = get_day_num(file!());
    println!("{}", file!());
    println!("Day {day}.1 Solution: {:?}", part_one(&input));
    println!("Day {day}.2 Solution: {:?}", part_two(&input));
}

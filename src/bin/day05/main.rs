use aoc2023::{get_day_str, read_input};
use std::time::Instant;

fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    // Extract starting seeds.
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok());
    lines.next(); // skip blank line

    // Read each map.
    let mut mapper_fns = Vec::<Box<dyn Fn(u64) -> u64>>::new();
    loop {
        let line = lines.next(); // map title, or end of input
        println!("Skipped: {:?}", &line);
        if line.is_none() {
            break;
        }

        let maps: Vec<_> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                dbg!(line);
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        let mapper_fn = move |n: u64| {
            for map in maps.clone() {
                let [dest_start, src_start, count] = map[..].try_into().unwrap();

                if src_start <= n && n < src_start + count {
                    let result = n as i64 + (dest_start as i64 - src_start as i64);
                    return result as u64;
                }
            }
            n
        };

        mapper_fns.push(Box::new(mapper_fn));
    }

    // Call the sequence of mapper functions on each seed.
    let mut outputs = Vec::new();
    for mut seed in seeds {
        println!("Starting seed: {}", seed);
        for f in mapper_fns.iter() {
            print!("{} -> ", seed);
            seed = f(seed);
            println!("{}", &seed);
        }
        outputs.push(seed);
    }

    outputs.into_iter().min()
}

fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    // Extract starting seed info.
    let seed_info: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    lines.next(); // skip blank line

    // Read each map, applying the mapping rules in reverse.
    let mut mapper_fns = Vec::<Box<dyn Fn(u64) -> u64>>::new();
    loop {
        let line = lines.next(); // map title, or end of input
        println!("Skipped: {:?}", &line);
        if line.is_none() {
            break;
        }

        let maps: Vec<_> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                dbg!(line);
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        let mapper_fn = move |n: u64| {
            for map in maps.clone() {
                let [src_start, dest_start, count] = map[..].try_into().unwrap();

                if src_start <= n && n < src_start + count {
                    let result = n as i64 + (dest_start as i64 - src_start as i64);
                    return result as u64;
                }
            }
            n
        };

        mapper_fns.push(Box::new(mapper_fn));
    }

    // Call the sequence of mapper functions in reverse on incrementing numbers until we reach a seed.
    let mut candidate: u64 = 0;
    let is_seed = |n: u64| {
        let mut info = seed_info.iter();
        while let Some(start) = info.next() {
            let count = info.next().unwrap();
            if *start <= n && n < start + count {
                return true;
            }
        }
        false
    };

    loop {
        let mut value = candidate;
        for f in mapper_fns.iter().rev() {
            value = f(value);
        }
        if is_seed(value) {
            return Some(candidate);
        } else {
            candidate += 1;
        }
    }
}

fn main() {
    let input = read_input(file!(), "input.txt");
    let day = get_day_str(file!());

    let time = Instant::now();
    println!(
        "{day}-1 solution: {:?} | {:.2?}",
        part_one(&input),
        time.elapsed()
    );

    let time = Instant::now();
    println!(
        "{day}-2 solution: {:?} | {:.2?}",
        part_two(&input),
        time.elapsed()
    );
}

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

const TOWEL: usize = usize::MAX;

fn get_combinations(design: &str, designs: &mut HashMap<String, usize>) -> usize {
    let mut count = 0;

    let (has_value, value) = match designs.get(design) {
        Some(x) => (true, *x),
        None => (false, 0),
    };

    if has_value {
        // if the design is cached, return it
        if value != TOWEL {
            return value;
        }
        else {
            count += 1;
        }
    }
 
    for i in 1..design.len() {
        if designs.get(&design[..i]) == Some(&TOWEL) {
            count += get_combinations(&design[i..], designs);
        }
    }

    // cache the design
    if !has_value {
        designs.insert(design.to_string(), count);
    }

    return count;
}

fn main() {
    let mut input = BufReader::new(File::open("input.txt").unwrap());

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    // read towels
    let mut buffer = String::new();
    let _ = input.read_line(&mut buffer);
    let _ = input.read_line(&mut buffer);

    let towels = buffer[..buffer.len()-2].split(", ").map(|x| (x.to_string(), TOWEL));
    let mut designs= HashMap::from_iter(towels);

    // read designs
    for line in input.lines() {
        let design= line.unwrap();
        let combinations = get_combinations(&design, &mut designs);

        part1_answer += (combinations != 0) as usize;
        part2_answer += combinations;
    }

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
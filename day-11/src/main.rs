use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut stones: HashMap<usize, usize> = HashMap::from_iter(input.split_whitespace().map(|x| (x.parse().unwrap(), 1)));
    let mut new_stones: HashMap<usize, usize> = HashMap::new();

    for blink in 1..=75 {
        for (stone, count) in &mut stones {
            let count_copy = *count;

            if count_copy == 0 {
                continue;
            }

            *count = 0;

            if *stone == 0 {
                *new_stones.entry(1).or_default() += count_copy;
                continue;
            }

            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let denominator = 10usize.pow(digits / 2);
                let left = stone / denominator;
                let right = stone % denominator;

                *new_stones.entry(left).or_default() += count_copy;
                *new_stones.entry(right).or_default() += count_copy;
                continue;
            }

            *new_stones.entry(stone * 2024).or_default() += count_copy;
        }

        stones.extend(&new_stones);
        new_stones.clear();

        if blink == 25 {
            println!("Part 1: {}", stones.values().sum::<usize>());
        }
    }

    println!("Part 2: {:?}", stones.values().sum::<usize>());
}
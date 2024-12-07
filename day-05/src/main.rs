use std::{cmp::Ordering, collections::{HashMap, HashSet}, io::BufRead};

fn is_ordered(pages: &Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    for i in 1..pages.len() {
        let (first, second) = (pages[i-1], pages[i]);

        if rules[&second].contains(&first) {
            return false;
        }

        if rules[&first].contains(&second) {
            continue;
        }
    }

    return true;
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let mut lines = std::io::BufReader::new(file).lines().flatten();

    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    // process the rules
    for line in lines.by_ref() {
        if line.is_empty() {
            // we've finished reading the rules
            break;
        }

        let pages: Vec<usize> = line.split('|').map(|x| x.parse().unwrap()).collect();
        assert!(pages.len() == 2);

        rules.entry(pages[0]).or_default().insert(pages[1]);
    }

    // process the updates
    for line in lines.by_ref() {
        let pages: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();

        if is_ordered(&pages, &rules) {
            let middle_number = pages[pages.len() / 2];
            part1_sum += middle_number;

        } else {
            // try to fix order of updates
            let mut pages_copy = pages.clone();

            pages_copy.sort_by( |a, b|
                if rules[a].contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            );

            if is_ordered(&pages_copy, &rules) {
                let middle_number = pages_copy[pages_copy.len() / 2];
                part2_sum += middle_number;
            }
        }
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}
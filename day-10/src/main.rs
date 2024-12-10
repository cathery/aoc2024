use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

fn search_paths(pos: Point, height: usize, map: &Vec<Vec<usize>>, results: &mut HashMap<Point, usize>) {

    if map[pos.y][pos.x] != height {
        return;
    }

    if height == 9 {
        *results.entry(pos).or_default() += 1;
        return;
    }

    if pos.y != 0 {
        search_paths(Point{x: pos.x, y: pos.y - 1}, height + 1, map, results);
    }
    if pos.x != 0 {
        search_paths(Point{x: pos.x - 1, y: pos.y}, height + 1, map, results);
    }
    if pos.y != map.len() - 1 {
        search_paths(Point{x: pos.x, y: pos.y + 1}, height + 1, map, results);
    }
    if pos.x != map[pos.y].len() - 1 {
        search_paths(Point{x: pos.x + 1, y: pos.y}, height + 1, map, results);
    }
}

fn main() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut starts: Vec<Point> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (x, ch) in line.unwrap().char_indices() {
            let digit = ch.to_digit(10).unwrap() as usize;

            if digit == 0 {
                starts.push(Point{x, y});
            }

            map[y].push(digit);
        }
    }

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut results = HashMap::new();

    for start in starts {
        search_paths(start, 0, &map, &mut results);

        part1_sum += results.len();
        part2_sum += results.values().sum::<usize>();
        results.clear();
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}
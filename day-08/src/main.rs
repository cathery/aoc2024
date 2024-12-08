use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Point {
        Point{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

fn main() {
    let lines: Vec<String> = std::fs::read_to_string("input.txt").unwrap().lines().map(String::from).collect();

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut part1_antinodes: HashSet<Point> = HashSet::new();
    let mut part2_antinodes: HashSet<Point> = HashSet::new();

    let max_y = (lines.len() - 1) as i32;
    let max_x = (lines[0].len() - 1) as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch != '.' {
                antennas.entry(ch).or_default().push(Point{x: x as i32,y: y as i32});
            }
        }
    }

    for (_antenna, positions) in antennas {
        for first in &positions {
            for second in &positions {
                if first == second {
                    continue;
                }

                let diff = second - first;

                // project antinodes from second antenna in the direction of first
                let mut current_pos = second.clone();
                let mut index = 0;
                loop {
                    current_pos = &current_pos - &diff;

                    if (0 <= current_pos.x && current_pos.x <= max_x)
                    && (0 <= current_pos.y && current_pos.y <= max_y) {
                        part2_antinodes.insert(current_pos);

                        // if we're at the second antinode, add it to part 1
                        if index == 1 {
                            part1_antinodes.insert(current_pos);
                        }

                        index += 1;
                    }
                    else {
                        break;
                    }
                }

            }
        }
    }

    println!("Part 1: {}", part1_antinodes.len());
    println!("Part 2: {}", part2_antinodes.len());
}
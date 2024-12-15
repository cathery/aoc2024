use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Clone, Copy, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn moved(&self, dir: Direction, mag: usize) -> Point {
        match dir {
            Up => Point{x: self.x, y: self.y - mag},
            Down => Point{x: self.x, y: self.y + mag},
            Left => Point{x: self.x - mag, y: self.y},
            Right => Point{x: self.x + mag, y: self.y},
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Block {
    Empty,
    Crate,
    CrateLeft,
    CrateRight,
    Wall,
}
use Block::*;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

fn walk_part1(pos: &mut Point, dir: Direction, map: &mut Vec<Vec<Block>>) {
    let mut i = 1;
    loop {
        let next_pos = pos.moved(dir, i);
        let next_block = map[next_pos.y][next_pos.x];

        match next_block {
            Crate => {
                i += 1;
                continue;
            },

            Empty => {
                // if the next block is empty, swap the last block with the first block,
                // and move the robot to the first block
                let first_pos = pos.moved(dir, 1);
                let first_block = map[first_pos.y][first_pos.x];

                map[next_pos.y][next_pos.x] = first_block;
                map[first_pos.y][first_pos.x] = Empty;
                *pos = first_pos;
            },

            _ => {}
        }

        return;
    }
}

fn walk_part2(pos: &mut Point, dir: Direction, map2: &mut Vec<Vec<Block>>) {
    let target_pos = pos.moved(dir, 1);
    let target_block = map2[target_pos.y][target_pos.x];
    match target_block {

        Empty => {
            *pos = target_pos;
        },

        CrateLeft | CrateRight => {
            let is_right = (target_block == CrateRight) as usize;
            let crate_pos = target_pos.moved(Left, is_right);
            if can_move_crate(&crate_pos, dir, map2) {
                move_crates(&crate_pos, dir, map2);

                *pos = target_pos;
            }
        },

        _ => {},
    }
}

// Position must be the left half of the crate
// Goes recursively through every crate in the direction and checks if the original crate can be moved
fn can_move_crate(pos: &Point, dir: Direction, map2: &mut Vec<Vec<Block>>) -> bool {
    let next_left_pos = pos.moved(dir, 1);
    let next_right_pos = next_left_pos.moved(Right, 1);
    let next_left_block = map2[next_left_pos.y][next_left_pos.x];
    let next_right_block = map2[next_right_pos.y][next_right_pos.x];

    match dir {
        Left => {
            match next_left_block {
                Empty => return true,
                Wall => return false,
                CrateRight => return can_move_crate(&pos.moved(Left, 2), dir, map2),
                _ => panic!(),
            }
        }
        Right => {
            match next_right_block {
                Empty => return true,
                Wall => return false,
                CrateLeft => return can_move_crate(&pos.moved(Right, 2), dir, map2),
                _ => panic!(),
            }
        }
        Up | Down => {
            if next_left_block  == Wall
            || next_right_block == Wall {
                return false;
            }
    
            if next_left_block == CrateLeft {
                if !can_move_crate(&next_left_pos, dir, map2) {
                    return false;
                }
            }
    
            if next_left_block == CrateRight {
                if !can_move_crate(&next_left_pos.moved(Left, 1), dir, map2) {
                    return false;
                }
            }
    
            if next_right_block == CrateLeft {
                if !can_move_crate(&next_right_pos, dir, map2) {
                    return false;
                }
            }
    
            return true;
        }
    }
}

// Position must be the left half of the crate
// Recursively moves every crate in the direction
fn move_crates(pos: &Point, dir: Direction, map2: &mut Vec<Vec<Block>>) {
    let next_left_pos = pos.moved(dir, 1);
    let next_right_pos = next_left_pos.moved(Right, 1);
    let next_left_block = map2[next_left_pos.y][next_left_pos.x];
    let next_right_block = map2[next_right_pos.y][next_right_pos.x];

    match dir {
        Left => {
            match next_left_block {
                CrateLeft | Wall => panic!(),
                CrateRight => move_crates(&pos.moved(Left, 2), dir, map2),
                _ => {},
            }

            map2[pos.y][pos.x +1] = Empty;
            map2[next_left_pos.y][next_left_pos.x] = CrateLeft;
            map2[next_right_pos.y][next_right_pos.x] = CrateRight;
            return;
        },
        Right => {
            match next_right_block {
                CrateRight | Wall => panic!(),
                CrateLeft => move_crates(&next_right_pos, dir, map2),
                _ => {},
            }
    
            map2[pos.y][pos.x] = Empty;
            map2[next_left_pos.y][next_left_pos.x] = CrateLeft;
            map2[next_right_pos.y][next_right_pos.x] = CrateRight;
            return;
        }
        Up | Down => {
            if next_left_block == Wall
            || next_right_block == Wall {
                panic!();
            }
    
            if next_left_block == CrateLeft {
                move_crates(&next_left_pos, dir, map2);
            }
    
            if next_left_block == CrateRight {
                move_crates(&next_left_pos.moved(Left, 1), dir, map2);
            }
    
            if next_right_block == CrateLeft {
                move_crates(&next_right_pos, dir, map2);
            }
    
            map2[pos.y][pos.x] = Empty;
            map2[pos.y][pos.x +1] = Empty;
            map2[next_left_pos.y][next_left_pos.x] = CrateLeft;
            map2[next_right_pos.y][next_right_pos.x] = CrateRight;
            return;
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let mut input = BufReader::new(File::open("input.txt").unwrap());
    let mut buffer: String = String::new();

    let mut map_part1: Vec<Vec<Block>> = Vec::new();
    let mut map_part2: Vec<Vec<Block>> = Vec::new();

    let mut position_part1: Point = Point {x: 0, y: 0};
    let mut position_part2: Point = Point {x: 0, y: 0};

    let mut y = 0;
    loop {
        buffer.clear();
        let _ = input.read_line(&mut buffer);

        if buffer == "\n" {
            break;
        }

        map_part1.push(Vec::with_capacity(buffer.len()));
        map_part2.push(Vec::with_capacity(buffer.len() * 2));
        for (x, ch) in buffer.char_indices() {
            if ch == '\n' {
                continue;
            }

            match ch {
                'O' => {
                    map_part1[y].push(Crate);

                    map_part2[y].push(CrateLeft);
                    map_part2[y].push(CrateRight);
                    continue;
                },
                '#' => {
                    map_part1[y].push(Wall);

                    map_part2[y].push(Wall);
                    map_part2[y].push(Wall);
                    continue;
                },
                '@' => {
                    position_part1 = Point{x: x, y: y};

                    position_part2 = Point{x: (x*2), y: y};
                    // fallthrough to empty
                },
                _ => {}
            }

            map_part1[y].push(Empty);

            map_part2[y].push(Empty);
            map_part2[y].push(Empty);
        }
        y += 1;
    }

    for line in input.lines() {
        for ch in line.unwrap().chars() {
            let dir = match ch {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                _   => Direction::Right,
            };
            walk_part1(&mut position_part1, dir, &mut map_part1);
            walk_part2(&mut position_part2, dir, &mut map_part2);
        }
    }

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for (y, line) in map_part1.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            if *block == Crate {
                part1_sum += 100 * y + x;
            }
        }
    }

    for (y, line) in map_part2.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            if *block == CrateLeft {
                part2_sum += 100 * y + x;
            }
        }
    }

    let elapsed = now.elapsed();

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
    println!("Solved in {:?}", elapsed);
}
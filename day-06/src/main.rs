use std::io::BufRead;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

fn rotate(dir: Direction) -> Direction {
    return match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_next_pos(pos: &Point<usize>, dir: Direction, map: &Vec<Vec<usize>>) -> Point<usize> { 
    return match dir {
        Direction::Up => {
            if pos.y == 0 {
                return *pos;
            }

            Point{x: pos.x, y: pos.y - 1}
        },
        Direction::Right => {
            if pos.x == map[pos.y].len() - 1 {
                return *pos;
            }

            Point{x: pos.x + 1, y: pos.y}
        },
        Direction::Down => {
            if pos.y == map.len() - 1 {
                return *pos;
            }

            Point{x: pos.x, y: pos.y + 1}
        },
        Direction::Left => {
            if pos.x == 0 {
                return *pos;
            }

            Point{x: pos.x - 1, y: pos.y}
        },
    }
}

fn try_loop(pos: &Point<usize>, dir: Direction, block_pos: &Point<usize>, map: &Vec<Vec<usize>>) -> bool {

    // walk through the map
    let mut cur_pos = pos.clone();
    let mut cur_dir = dir.clone();
    let mut turns_count = 0;
    loop {
        let next_pos = get_next_pos(&cur_pos, cur_dir, &map);
        if next_pos == cur_pos {
            return false;
        }

        if (next_pos == *block_pos) || (map[next_pos.y][next_pos.x] == 1000) {
            cur_dir = rotate(cur_dir);
            turns_count += 1;

            // if we have made this many turns, we're probably in a loop
            if turns_count >= 300 {
                return true;
            }
            continue;
        }

        cur_pos = next_pos;
    }
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines().flatten();

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    let mut map: Vec<Vec<usize>> = Vec::new();

    let mut guard_pos: Point<usize> = Point{x: 0, y: 0};
    let mut guard_dir: Direction = Direction::Up;

    // read the map
    for (y, line) in lines.enumerate() {
        map.push(Vec::new());
        for (x, block) in line.char_indices() {
            if block == '^' {
                guard_pos.x = x;
                guard_pos.y = y;
            }

            let value: usize = match block {
                '#' => 1000,
                _   => 1,
            };
            map[y].push(value);
        }
    }

    // Try every position for an obstacle and see if they loop
    for (y, line) in map.iter().enumerate() {
        for (x, _dir) in line.iter().enumerate() {

            let block_pos = Point{x,y};
            if try_loop(&guard_pos, guard_dir, &block_pos, &map) {
                part2_sum += 1;
            }
        }
    }

    // walk through the map
    loop {
        part1_sum += map[guard_pos.y][guard_pos.x];
        
        map[guard_pos.y][guard_pos.x] = 0;

        let next_pos = get_next_pos(&guard_pos, guard_dir, &map);
        if next_pos == guard_pos {
            break;
        }

        if map[next_pos.y][next_pos.x] == 1000 {
            guard_dir = rotate(guard_dir);
            continue;
        }

        guard_pos = next_pos;
    }

    println!("Part 1: {:?}", part1_sum);
    println!("Part 2: {:?}", part2_sum);
}
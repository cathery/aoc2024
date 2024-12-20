use std::{collections::{HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};

type Position = (usize, usize);
type Cheat = (Position, Position);

type Block = usize;
const WALL: Block = Block::MAX;

// Simple pathfinding algorithm
// Each tile is marked with how many steps away it is from the start
fn walk((start_x, start_y): Position, map: &mut Vec<Vec<Block>>) {
    let mut positions: VecDeque<Position> = VecDeque::from([(start_x, start_y)]);

    while let Some((pos_x, pos_y)) = positions.pop_front() {
        let score = map[pos_y][pos_x] + 1;

        macro_rules! try_position {
            ($x:expr, $y:expr) => {
                let adj_score = map[$y][$x];
                if adj_score != WALL && score < adj_score {
                    map[$y][$x] = score;
                    positions.push_back(($x, $y));
                }
            };
        }

        try_position!(pos_x, pos_y - 1);
        try_position!(pos_x, pos_y + 1);
        try_position!(pos_x - 1, pos_y);
        try_position!(pos_x + 1, pos_y);
    }
}

fn get_cheats(map: &Vec<Vec<Block>>) -> HashSet<Cheat> {
    let mut cheats: HashSet<Cheat> = HashSet::new();

    for (y, line) in map.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            if *block == WALL {
                continue;
            }

            let start_pos = (x, y);
            let start_time = *block;

            macro_rules! try_cheat {
                ($x:expr, $y:expr) => {
                    let end_time = map[$y][$x];
                    if end_time != WALL {
                        let distance = $x.abs_diff(x) + $y.abs_diff(y);

                        // if there is time to be saved, add this as a valid cheat
                        if start_time + distance < end_time {
                            cheats.insert((start_pos, ($x, $y)));
                        }
                    }
                };
            }

            for cheat_length in 2..=20 {
                for dy in 0..=cheat_length {
                    let dx = cheat_length - dy;

                    let yminus = y > dy;
                    let xminus = x > dx;
                    let yplus = dy != 0 && y + dy < map.len();
                    let xplus = dx != 0 && x + dx < map[y].len();

                    if xplus && yplus {
                        try_cheat!(x + dx, y + dy);
                    }

                    if xminus && yplus {
                        try_cheat!(x - dx, y + dy);
                    }

                    if xplus && yminus {
                        try_cheat!(x + dx, y - dy);
                    }

                    if xminus && yminus {
                        try_cheat!(x - dx, y - dy);
                    }
                }
            }
        }
    }

    return cheats;
}

fn main() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let mut map: Vec<Vec<Block>> = Vec::new();
    let mut start_pos: Position = Position::default();

    for (y, line_r) in input.lines().enumerate() {
        let line = line_r.unwrap();
        map.push(Vec::with_capacity(line.len()));

        for (x, ch) in line.char_indices() {
            match ch {
                '#' => map[y].push(WALL),
                'S' => {
                    start_pos = (x, y);
                    map[y].push(0);
                },
                _ => map[y].push(usize::MAX - 1),
            }
        }
    }

    walk(start_pos, &mut map);

    let cheats: HashSet<Cheat> = get_cheats(&mut map);

    for (start_pos, end_pos) in cheats {
        let distance = end_pos.0.abs_diff(start_pos.0) + end_pos.1.abs_diff(start_pos.1);
        let old_time = map[start_pos.1][start_pos.0];
        let new_time = map[end_pos.1][end_pos.0];
        let timesave = new_time - (old_time + distance);
        
        if timesave < 100 {
            continue;
        }
        
        part2_answer += 1;
        if distance == 2 {
            part1_answer += 1;
        }
    }

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
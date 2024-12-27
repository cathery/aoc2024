use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}};

type Direction = u8;
const UP:    Direction = 0;
const RIGHT: Direction = 1;
const DOWN:  Direction = 2;
const LEFT:  Direction = 3;

type Score = usize;

type Position = (usize, usize);

fn walk(start_pos: Position, end_pos: Position, walls: Vec<Vec<bool>>) -> (usize, usize) {
    let mut queue: VecDeque<(Position, Direction, usize, Vec<Position>)> = VecDeque::from([(start_pos, RIGHT, 0, Vec::from([start_pos]))]);

    let mut map: HashMap<(Position, Direction), Score> = Default::default();

    let mut best_score = usize::MAX;
    let mut best_paths: HashMap<Score, HashSet<Position>> = Default::default();

    while let Some(((pos_x, pos_y), dir, score, path)) = queue.pop_front() {
        macro_rules! try_position {
            ($x:expr, $y:expr, $d:expr, $s:expr) => {
                let pos = ($x, $y);
                if !walls[$y][$x] {
                    let new_score = score + $s;
                    if new_score <= best_score {
                        let adj_score = map.entry((pos, dir)).or_insert(new_score);
                        if new_score <= *adj_score {
                            *adj_score = new_score;

                            let mut new_path = path.clone();
                            new_path.push(pos);

                            if pos == end_pos {
                                best_score = new_score;
                                best_paths.entry(best_score).or_default().extend(new_path);
                            }
                            else {
                                queue.push_back((pos, $d, new_score, new_path));
                            }
                        }
                    }
                }
            };
        }

        match dir {
            UP => {
                try_position!(pos_x, pos_y - 1, UP,       1);
                try_position!(pos_x + 1, pos_y, RIGHT, 1001);
                try_position!(pos_x - 1, pos_y, LEFT,  1001);
            },
            DOWN => {
                try_position!(pos_x, pos_y + 1, DOWN,     1);
                try_position!(pos_x - 1, pos_y, LEFT,  1001);
                try_position!(pos_x + 1, pos_y, RIGHT, 1001);
            },
            LEFT => {
                try_position!(pos_x - 1, pos_y, LEFT,    1);
                try_position!(pos_x, pos_y - 1, UP,   1001);
                try_position!(pos_x, pos_y + 1, DOWN, 1001);
            }
            RIGHT => {
                try_position!(pos_x + 1, pos_y, RIGHT,   1);
                try_position!(pos_x, pos_y + 1, DOWN, 1001);
                try_position!(pos_x, pos_y - 1, UP,   1001);
            },
            _ => panic!(),
        }
    }

    return (best_score, best_paths[&best_score].len());
}

fn main() {
    let now = std::time::Instant::now();
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut start_pos: Position = (0,0);
    let mut end_pos: Position = (0,0);

    let mut walls: Vec<Vec<bool>> = Vec::new();

    for (y, line_r) in input.lines().enumerate() {
        let line = line_r.unwrap();
        walls.push(vec!(false; line.len()));

        for (x, ch) in line.char_indices() {
            match ch {
                'S' => { 
                    start_pos = (x, y)
                },
                'E' => {
                    end_pos = (x,y)
                },
                '#' => {
                    walls[y][x] = true
                },
                _ => {}
            };
        }
    }

    let (part1_score, part2_score) = walk(start_pos, end_pos, walls);
    let elapsed = now.elapsed();

    println!("Part 1: {}", part1_score);
    println!("Part 2: {}", part2_score);
    println!("Solved in {:?}", elapsed);
}
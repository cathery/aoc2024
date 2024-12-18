use std::{fs::File, io::{BufRead, BufReader}};

// only modify these 3 variables to fit the input
const GRID_START:  usize = 0;
const GRID_END:    usize = 70;
const PART1_BYTES: usize = 1024;

const GRID_SIZE:   usize = GRID_END - GRID_START + 1;
const BORDER_SIZE: usize = 1;
const MAP_SIZE:    usize = GRID_SIZE + BORDER_SIZE*2;

const START_X: usize = 0 + BORDER_SIZE;
const START_Y: usize = 0 + BORDER_SIZE;
const END_X:   usize = MAP_SIZE - BORDER_SIZE - 1;
const END_Y:   usize = MAP_SIZE - BORDER_SIZE - 1;

type Score = usize;
const BLOCKED:   Score = Score::MAX;
const UNVISITED: Score = Score::MAX - 1;

type Map = [[Score; MAP_SIZE]; MAP_SIZE];

fn walk(map: &mut Map) -> Score {
    let mut positions: Vec<(usize, usize)> = Vec::new();
    let mut new_positions: Vec<(usize, usize)> = Vec::new();

    map[START_Y][START_X] = 0;
    positions.push((START_X, START_Y));

    while positions.is_empty() == false {
        for (pos_x, pos_y) in &positions {
            let score = map[*pos_y][*pos_x] + 1;

            macro_rules! move_dir {
                (UP)    => {(*pos_x, *pos_y - 1)};
                (DOWN)  => {(*pos_x, *pos_y + 1)};
                (LEFT)  => {(*pos_x - 1, *pos_y)};
                (RIGHT) => {(*pos_x + 1, *pos_y)};
            }

            macro_rules! check_dir {
                ($d:tt) => {
                    let (new_x, new_y) = move_dir!($d);
                    let adj_score = map[new_y][new_x];
                    if adj_score != BLOCKED && score < adj_score {
                        map[new_y][new_x] = score;
                        new_positions.push((new_x, new_y));
                    }
                };
            }

            check_dir!(UP);
            check_dir!(DOWN);
            check_dir!(LEFT);
            check_dir!(RIGHT);
        }

        positions.clear();
        positions.append(&mut new_positions);
    }

    return map[END_Y][END_X];
}

fn main() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut map: Map = [[UNVISITED; MAP_SIZE]; MAP_SIZE];
    let mut part1_answer = 0;
    let mut part2_answer = (0,0);

    // draw borders around the map to avoid bound checks
    for i in 0..MAP_SIZE {
        map[i][0] = BLOCKED;
        map[0][i] = BLOCKED;
        map[MAP_SIZE-1][i] = BLOCKED;
        map[i][MAP_SIZE-1] = BLOCKED;
    }

    let mut byte_count = 0;
    for line in input.lines() {
        let numbers: Vec<usize> = line.unwrap().split(',').flat_map(|x| x.parse()).collect();
        let (x, y) = (numbers[0], numbers[1]);

        map[y - GRID_START + START_Y][x - GRID_START + START_X] = BLOCKED;

        byte_count += 1;

        if byte_count < PART1_BYTES {
            continue;
        }

        let score = walk(&mut map.clone());

        if byte_count == PART1_BYTES {
            part1_answer = score;
        }

        if score == UNVISITED {
            part2_answer = (x,y);
            break;
        }
    }

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {},{}", part2_answer.0, part2_answer.1);
}
use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

fn main() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    const WIDTH: i32       = 101;
    const HEIGHT: i32      = 103;
    const HALF_WIDTH: i32  = WIDTH / 2;
    const HALF_HEIGHT: i32 = HEIGHT / 2;
    const SECONDS: i32     = 100;


    let mut robots: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut quadrants = [0; 4];

    for line in input.lines() {
        let numbers: Vec<i32> = line.unwrap().split(['=',',',' ']).flat_map(|x| x.parse()).collect();
        let (pos_x, pos_y, vel_x, vel_y) = (numbers[0], numbers[1], numbers[2], numbers[3]);

        robots.push((pos_x, pos_y, vel_x, vel_y));

        let result_x = (pos_x + (vel_x * (SECONDS))).rem_euclid(WIDTH);
        let result_y = (pos_y + (vel_y * (SECONDS))).rem_euclid(HEIGHT);

        if result_x == HALF_WIDTH
        || result_y == HALF_HEIGHT {
            continue;
        }

        let is_right =  (result_x-1) / HALF_WIDTH;
        let is_bottom = (result_y-1) / HALF_HEIGHT;
        let quadrant = (is_right * 2 + is_bottom) as usize;

        quadrants[quadrant] += 1;
    }

    let mut positions: HashSet<i32> = HashSet::new();
    let mut secs = 0;
    'outer: loop {
        positions.clear();
        for (pos_x, pos_y, vel_x, vel_y) in &robots {
            let result_x = (pos_x + (vel_x * (secs))).rem_euclid(WIDTH);
            let result_y = (pos_y + (vel_y * (secs))).rem_euclid(HEIGHT);

            let position = result_x * WIDTH + result_y;

            // if any two robots are ever on the same tile, skip to the next second
            if !positions.insert(position) {
                secs += 1;
                continue 'outer;
            }
        }

        // all robots are in unique positions, this must be the right answer
        break;
    }

    let part1: usize = quadrants.iter().product();
    let part2 = secs;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
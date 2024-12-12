use std::{fs::File, io::{BufRead, BufReader}};

type Plot = (char, bool);  // (plot_type, visited)
type Map = Vec<Vec<Plot>>;

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

fn walk_through_plots((x, y) : (usize, usize), plot_type: char, plots: &mut Map) -> (usize, usize, usize) {
    // mark as visited
    plots[y][x].1 = true;

    let mut total_area = 1;
    let mut total_perimeter = 0;
    let mut total_sides = 0;

    let has_above: bool = y != 0;
    let has_below: bool = y != plots.len() - 1;
    let has_left: bool = x != 0;
    let has_right: bool = x != plots[y].len() - 1;

    let has_neighbor = [
        has_above && plots[y-1][x].0 == plot_type,
        has_below && plots[y+1][x].0 == plot_type,
        has_left  && plots[y][x-1].0 == plot_type,
        has_right && plots[y][x+1].0 == plot_type,
    ];

    let has_diagonal = [
        has_above && has_left  && plots[y-1][x-1].0 == plot_type,
        has_above && has_right && plots[y-1][x+1].0 == plot_type,
        has_below && has_left  && plots[y+1][x-1].0 == plot_type,
        has_below && has_right && plots[y+1][x+1].0 == plot_type
    ];

    let has_corner = [
        has_neighbor[UP]   == has_neighbor[LEFT]  && (!has_diagonal[UP]    || !has_neighbor[UP]),
        has_neighbor[UP]   == has_neighbor[RIGHT] && (!has_diagonal[DOWN]  || !has_neighbor[UP]),
        has_neighbor[DOWN] == has_neighbor[LEFT]  && (!has_diagonal[LEFT]  || !has_neighbor[DOWN]),
        has_neighbor[DOWN] == has_neighbor[RIGHT] && (!has_diagonal[RIGHT] || !has_neighbor[DOWN]),
    ];

    for i in has_corner {
        total_sides += i as usize;
    }
    for i in has_neighbor {
        total_perimeter += (!i) as usize;
    }

    macro_rules! dir {
        (UP) => {(x, y-1)};
        (DOWN) => {(x, y+1)};
        (LEFT) => {(x-1, y)};
        (RIGHT) => {(x+1, y)};
    }

    macro_rules! walk {
        ($i:tt) => {
            if has_neighbor[$i] {
                let dir = dir!($i);
                // if not visited yet
                if !plots[dir.1][dir.0].1 {
                    let (area, perimeter, sides) = walk_through_plots(dir, plot_type, plots);
                    total_area += area;
                    total_perimeter += perimeter;
                    total_sides += sides;
                }
            }
            
        }
    }

    walk!(UP);
    walk!(DOWN);
    walk!(LEFT);
    walk!(RIGHT);

    return (total_area, total_perimeter, total_sides);
}

fn main() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut plots: Map = Map::new();

    for (y, line) in input.lines().enumerate() {
        plots.push(Vec::new());
        for ch in line.unwrap().chars() {
            plots[y].push((ch, false));
        }
    }

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for y in 0..plots.len() {
        for x in 0..plots[y].len() {
            let (ch, visited) = plots[y][x];
            if !visited {
                let (area, perimeter, sides) = walk_through_plots((x, y), ch, &mut plots);
                part1_sum += area * perimeter;
                part2_sum += area * sides;
            }
        }
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}
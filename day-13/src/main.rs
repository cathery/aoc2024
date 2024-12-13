use std::{fs::File, io::{BufRead, BufReader}};

fn play_arcade(a_x: usize, a_y: usize, b_x: usize, b_y: usize, prize_x: usize, prize_y: usize) -> usize {
    let a_numerator = (b_x * prize_y).abs_diff(b_y * prize_x);
    let b_numerator = (a_x * prize_y).abs_diff(a_y * prize_x);

    let denominator = (a_x * b_y).abs_diff(a_y * b_x);

    if a_numerator % denominator == 0 
    && b_numerator % denominator == 0 {
        let a = a_numerator / denominator;
        let b = b_numerator / denominator;

        return a*3 + b;
    }

    return 0;
}

fn main() {
    let mut input = BufReader::new(File::open("input.txt").unwrap());
    let mut buffer = String::new();

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    loop {
        buffer.clear();
        let _ = input.read_line(&mut buffer);
        let _ = input.read_line(&mut buffer);
        let _ = input.read_line(&mut buffer);
        let numbers: Vec<usize> = buffer.split(['+',',','=','\n']).flat_map(|x| x.parse()).collect();

        let a_x     = numbers[0];
        let a_y     = numbers[1];
        let b_x     = numbers[2];
        let b_y     = numbers[3];
        let prize_x = numbers[4];
        let prize_y = numbers[5];

        part1_sum += play_arcade(a_x, a_y, b_x, b_y, prize_x, prize_y);
        part2_sum += play_arcade(a_x, a_y, b_x, b_y, prize_x + 10000000000000, prize_y + 10000000000000);

        if input.read_line(&mut buffer).unwrap() == 0 {
            break;
        }
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}
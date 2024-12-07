use std::io::BufRead;

fn concatenate(num1: usize, num2: usize) -> usize {
    return num1 * 10usize.pow(num2.ilog10() + 1) + num2;
}

fn try_sums(result: usize, values: &[usize], current_sum: usize, ops: usize) -> bool {
    let next_value = values[0];

    for op in 0..ops {
        let next_sum = match op {
            0 => current_sum + next_value,
            1 => current_sum * next_value,
            2 => concatenate(current_sum, next_value),
            _ => panic!()
        };

        if next_sum > result {
            continue;
        }

        if values.len() == 1 {
            if next_sum == result {
                return true;
            }

            continue;
        }

        if try_sums(result, values.split_first().unwrap().1, next_sum, ops) {
            return true;
        }
    }

    return false;
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines().flatten();

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for line in lines {
        let numbers: Vec<usize> = line.split([':',' ']).flat_map(|x| x.parse()).collect();

        let (result, values) = numbers.split_first().unwrap();

        // try part 1
        if try_sums(*result, values.split_first().unwrap().1, values[0], 2) {
            part1_sum += *result;
            part2_sum += *result;
        }
        // try part 2
        else if try_sums(*result, values.split_first().unwrap().1, values[0], 3) {
            part2_sum += result;
        }
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}
fn process_line(numbers: &Vec<u32>) -> Result<(), usize> {
    let is_increasing = numbers[0] < numbers[1];

    for i in 1..numbers.len() {
        let first_number = numbers[i - 1];
        let second_number = numbers[i];

        if first_number == second_number {
            // unsafe
            return Err(i);
        }

        if is_increasing && first_number > second_number {
            // unsafe
            return Err(i);
        }

        if !is_increasing && first_number < second_number {
            // unsafe
            return Err(i);
        }

        if first_number.abs_diff(second_number) > 3 {
            // unsafe
            return Err(i);
        }
    }

    return Ok(());
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for line in input.lines() {
        let numbers: Vec<u32> = line.split_whitespace().flat_map(|x| x.parse()).collect();

        let result = process_line(&numbers);

        if result.is_ok() {
            part1_sum += 1;
            part2_sum += 1;

            continue;
        }

        // try removing the first number in the array
        let test = numbers[1..].to_vec();

        if process_line(&test).is_ok() {
            part2_sum += 1;
            continue;
        }

        let error_index = result.unwrap_err();

        // try removing the first number in an erroneous pair
        let mut var1 = numbers.to_vec();
        var1.remove(error_index - 1);

        if process_line(&var1).is_ok() {
            part2_sum += 1;
            continue;
        }

        // try removing the second number in an erroneous pair
        let mut var2 = numbers.to_vec();
        var2.remove(error_index);

        if process_line(&var2).is_ok() {
            part2_sum += 1;
            continue;
        }
    }

    println!("Part 1: {part1_sum}");
    println!("Part 2: {part2_sum}");
}
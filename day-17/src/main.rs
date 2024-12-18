fn run_program(input: usize, program: &[usize]) -> Vec<usize> {
    let mut a = input;
    let mut b = 0; // assuming it doesn't matter
    let mut c = 0; // assuming it doesn't matter

    let mut ip = 0;
    let mut result: Vec<usize> = Vec::new();

    macro_rules! get_combo {
        () => (
            match program[ip + 1] {
                0..=3 => program[ip + 1],
                4 => a,
                5 => b,
                6 => c,
                _ => panic!()
            }
        )
    }

    macro_rules! get_literal { () => (program[ip + 1]) }

    loop {
        if ip >= program.len() {
            break;
        }

        let opcode = program[ip];
        match opcode {
            0 => { a >>= get_combo!() }
            1 => { b ^= get_literal!() }
            2 => { b = get_combo!() & 0b111 }
            3 => {
                if a != 0 {
                    ip = get_literal!();
                    continue;
                }
            }
            4 => { b ^= c }
            5 => { result.push(get_combo!() & 0b111) }
            6 => { b = a >> get_combo!() }
            7 => { c = a >> get_combo!() }
            _ => panic!()
        }
        ip += 2;
    }

    return result;
} 


fn solve_part2(answer: usize, digitcount: usize, program: &[usize]) -> Option<usize> {
    // assuming that the program reads input as octal,
    // and each input digit matches each program number

    // try every digit combination
    for digit in 0..0o10 {
        let input = answer + digit;
        let result = run_program(input, program);

        if result == program {
            return Some(input);
        }

        // if the output matches a part of the program, add a 0 to the answer and keep going
        if result == program[program.len() - digitcount..] {
            let output = solve_part2(input << 3, digitcount + 1, program);
            if output.is_some() {
                return output;
            }
        }
    }

    return None;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let numbers: Vec<usize> = input.split([' ',',','\n']).flat_map(|x| x.parse()).collect();
    let a_register = numbers[0];
    let program = &numbers[3..];

    let part1_result = run_program(a_register, program);
    let part2_result = solve_part2(0, 1, program);

    print!("Part 1: ");
    for (i, num) in part1_result.iter().enumerate() {
        match i {
            0 => print!("{}", num),
            _ => print!(",{}", num),
        }
    }
    println!();

    println!("Part 2: {:?}", part2_result.unwrap_or_default());
}
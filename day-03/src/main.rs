use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"(don't|do|mul)(?:\((\d+),(\d+)\)|\(()()\))").unwrap();

    let mut sum1 = 0;
    let mut sum2 = 0;

    let mut enabled = true;

    for (_, [command, param1, param2]) in re.captures_iter(&input).map(|c| c.extract()) {
        match command {
            "don't" => enabled = false,
            "do" => enabled = true,
            "mul" => {
                let mult = (*param1).parse::<i32>().unwrap() * (*param2).parse::<i32>().unwrap();

                sum1 += mult;

                if enabled {
                    sum2 += mult;
                }
            },
            _ => panic!()
        }
    }

    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");
}
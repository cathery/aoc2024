fn main() {
    let contents = std::fs::read_to_string("input.txt").unwrap();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in contents.lines() {
        let numbers: Vec<u32> = line.split_whitespace().flat_map(|x| x.parse()).collect();

        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }

    left_list.sort();
    right_list.sort();

    let pairs = std::iter::zip(left_list.iter(), right_list.iter());

    let mut sum = 0;
    let mut similarity_score = 0;

    for (left_number, right_number) in pairs {
        // let right_number = right_list[i];
        let difference = left_number.abs_diff(*right_number);
        let occurence_count = right_list.iter().filter(|x| *x == left_number).count() as u32;

        println!("Left: {left_number}, Right: {right_number}, diff: {difference}, occurences: {occurence_count}");

        sum += difference;
        similarity_score += left_number * occurence_count;
    }

    println!("Part 1: {sum}");
    println!("Part 2: {similarity_score}");

}

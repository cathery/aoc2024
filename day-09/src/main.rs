fn solve_part1(mut map: Vec<i32>) {

    let mut space_i = 0;
    'outer: for file_i in (0..map.len()).rev() {
        if map[file_i] == -1 {
            continue;
        }

        loop {
            if space_i == file_i {
                break 'outer;
            }

            if map[space_i] != -1 {
                space_i += 1;
                continue;
            }

            map.swap(file_i, space_i);
            space_i += 1;
            break;
        }
    }

    println!("Part 1: {}", get_checksum(&map));
}

fn solve_part2(mut map: Vec<i32>, mut id: i32) {
    // search for file from rightmost end
    let mut file_size = 0;
    for file_i in (0..map.len()).rev() {

        if map[file_i] == id {
            file_size += 1;

            if file_i > 0 && map[file_i - 1] == id {
                continue;
            }
        }

        if file_size == 0 {
            continue;
        }

        // if we're here, we found the entire file
        // search for space from leftmost end
        let mut space_size = 0;
        for space_i in 0..map.len() {
            // we only search for space to the left of the file
            if space_i == file_i {
                break;
            }

            if map[space_i] != -1 {
                space_size = 0;
                continue;
            }

            space_size += 1;

            if space_size < file_size {
                continue;
            }

            // we found enough space for the file, perform swap
            let (left, right) = map.split_at_mut(file_i);
            right[..file_size].swap_with_slice(&mut left[space_i-space_size+1..space_i+1]);
            break;
        }

        // after performing the swap, set the next file data
        id -= 1;
        file_size = 0;
    }

    println!("Part 2: {}", get_checksum(&map));
}

fn get_checksum(map: &Vec<i32>) -> usize {
    let mut sum= 0;

    for (pos, id) in map.iter().enumerate() {
        if *id == -1 {
            continue;
        }

        sum += pos * (*id as usize);
    }

    return sum;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut map: Vec<i32> = Vec::new();

    let mut last_id: i32 = 0;
    for (index, ch) in input.char_indices() {
        let count = ch.to_digit(10).unwrap() as usize;

        if index % 2 == 0 {
            last_id = (index / 2) as i32;
            for _ in 0..count {
                map.push(last_id);
            }
        }
        else {
            for _ in 0..count {
                map.push(-1);
            }
        }
    }

    solve_part1(map.clone());
    solve_part2(map, last_id);
}
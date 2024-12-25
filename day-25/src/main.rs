use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let mut input = BufReader::new(File::open("input.txt").unwrap());

    let mut keys: Vec<_> = Default::default();
    let mut locks: Vec<_> = Default::default();

    let mut buffer = [0; 6 * 7];
    loop {
        let res = input.read_exact(&mut buffer);
        let _ = input.seek_relative(1);
        if res.is_err() {
            break;
        }

        let mut pattern = [0; 5];
        let is_lock = (buffer[0] == b'#') as usize;
        for x in 0..5 {
            for y in 1..6 {
                let pos: usize = (is_lock * 6).abs_diff(y) * 6 + x;
                if buffer[pos] == b'#' && pattern[x] == 0 {
                    pattern[x] = 6 - y as u8;
                    break;
                }
            }
        }

        match is_lock {
            0 => keys.push(pattern),
            1 => locks.push(pattern),
            _ => panic!(),
        }
    }

    let mut answer = 0;
    for lock in locks.iter() {
        'keys: for key in keys.iter() {
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    continue 'keys;
                }
            }
            answer += 1;
        }
    }

    println!("Part 1: {}", answer);
}
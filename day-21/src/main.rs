use std::collections::HashMap;

type Sequence = usize;
const SEQ_UP:    Sequence = 0;
const SEQ_DOWN:  Sequence = 1;
const SEQ_LEFT:  Sequence = 2;
const SEQ_RIGHT: Sequence = 3;
const SEQ_A:     Sequence = 4;

type DigitSequence = usize;
const KEY_A: DigitSequence = 10;

const DIGIT_TO_SEQ_MAP: [[&str; 11]; 11] = [
//from  to|0       |1       |2      |3       |4       |5      |6      |7        |8       |9,      |A
/* 0 */  ["A"    , "^<A"  , "^A"  , "^>A"  , "^^<A" , "^^A" , "^^>A", "^^^<A" , "^^^A" , "^^^>A", ">A"    ],
/* 1 */  [">vA"  , "A"    , ">A"  , ">>A"  , "^A"   , "^>A" , "^>>A", "^^A"   , "^^>A" , "^^>>A", ">>vA"  ],
/* 2 */  ["vA"   , "<A"   , "A"   , ">A"   , "<^A"  , "^A"  , "^>A" , "<^^A"  , "^^A"  , "^^>A" , "v>A"   ],
/* 3 */  ["<vA"  , "<<A"  , "<A"  , "A"    , "<<^A" , "<^A" , "^A"  , "<<^^A" , "<^^A" , "^^A"  , "vA"    ],
/* 4 */  [">vvA" , "vA"   , "v>A" , "v>>A" , "A"    , ">A"  , ">>A" , "^A"    , "^>A"  , "^>>A" , ">>vvA" ],
/* 5 */  ["vvA"  , "<vA"  , "vA"  , "v>A"  , "<A"   , "A"   , ">A"  , "<^A"   , "^A"   , "^>A"  , "v>>A"  ],
/* 6 */  ["<vvA" , "<<vA" , "<vA" , "vA"   , "<<A"  , "<A"  , "A"   , "<<^A"  , "<^A"  , "^A"   , "vvA"   ],
/* 7 */  [">vvvA", "vvA"  , "vv>A", "vv>>A", "vA"   , "v>A" , "v>>A", "A"     , ">A"   , ">>A"  , ">>vvvA"],
/* 8 */  ["vvvA" , "<vvA" , "vvA" , "vv>A" , "<vA"  , "vA"  , "v>A" , "<A"    , "A"    , ">A"   , "vvv>A" ],
/* 9 */  ["<vvvA", "<<vvA", "<vvA", "vvA"  , "<<vA" , "<vA" , "vA"  , "<<A"   , "<A"   , "A"    , "vvvA"  ],
/* A */  ["<A"   , "^<<A" , "<^A" , "^A"   , "^^<<A", "<^^A", "^^A" , "^^^<<A", "<^^^A", "^^^A" , "A"     ],
];

const SEQ_TO_SEQ_MAP: [[&str; 5]; 5] = [
//from  to|^     |v     |<      |>     |A
/* ^ */  ["A"  , "vA" , "v<A" , "v>A", ">A"  ],
/* v */  ["^A" , "A"  , "<A"  , ">A" , "^>A" ],
/* < */  [">^A", ">A" , "A"   , ">>A", ">>^A"],
/* > */  ["<^A", "<A" , "<<A" , "A"  , "^A"  ],
/* A */  ["<A" , "<vA", "v<<A", "vA" , "A"   ],
];

fn get_password_pattern(password: &str, map: &Vec<Vec<Vec<Sequence>>>) -> Vec<Sequence> {
    let mut pattern: Vec<Sequence> = Vec::new();

    let mut prev_key = KEY_A;
    for letter in password.bytes() {
        let next_key = match letter {
            b'A' => KEY_A,
            _    => (letter - 0x30) as usize,
        };

        pattern.extend(&map[prev_key][next_key]);

        prev_key = next_key;
    }

    return pattern;
}

fn get_complexity(password: Vec<Sequence>, map: &Vec<Vec<Vec<Sequence>>>) -> (usize, usize) {
    let mut pattern_counts:     HashMap<(Sequence, Sequence), usize> = HashMap::new();
    let mut new_pattern_counts: HashMap<(Sequence, Sequence), usize> = HashMap::new();

    let mut prev_key = SEQ_A;
    for next_key in password {
        *pattern_counts.entry((prev_key,next_key)).or_default() += 1;
        prev_key = next_key;
    }

    let mut part1 = 0;

    for i in 0..25 {
        new_pattern_counts.clear();
        for ((from, to), count) in &pattern_counts {
            prev_key = SEQ_A;
            for next_key in &map[*from][*to] {
                *new_pattern_counts.entry((prev_key, *next_key)).or_default() += count;
                prev_key = *next_key;
            }
        }
        pattern_counts.clear();
        pattern_counts.extend(&new_pattern_counts);

        if i == 1 {
            part1 = pattern_counts.values().sum::<usize>();
        }
    }

    let part2 = pattern_counts.values().sum::<usize>();

    return (part1, part2);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    let mut digit_to_sequence_map = vec![vec![Vec::new(); DIGIT_TO_SEQ_MAP.len()]; DIGIT_TO_SEQ_MAP.len()];
    let mut sequence_to_sequence_map = vec![vec![Vec::new(); SEQ_TO_SEQ_MAP.len()]; SEQ_TO_SEQ_MAP.len()];

    macro_rules! convert_array {
        ($in:expr, $out:expr) => {
            for (from, patterns) in $in.into_iter().enumerate() {
                for (to, pattern) in patterns.into_iter().enumerate() {
                    for ch in pattern.bytes() {
                        let dir = match ch {
                            b'^' => SEQ_UP,
                            b'v' => SEQ_DOWN,
                            b'<' => SEQ_LEFT,
                            b'>' => SEQ_RIGHT,
                            b'A' => SEQ_A,
                            _ => panic!()
                        };
                        $out[from][to].push(dir);
                    }
                }
            }
        };
    }

    // convert string arrays to binary arrays (i dont think rust can do that at compile time)
    convert_array!(DIGIT_TO_SEQ_MAP, digit_to_sequence_map);
    convert_array!(SEQ_TO_SEQ_MAP, sequence_to_sequence_map);

    for line in input.lines() {
        let password: usize = line[..line.len()-1].parse().unwrap();
        let password_seq = get_password_pattern(line, &digit_to_sequence_map);
        let (part1_length, part2_length) = get_complexity(password_seq, &sequence_to_sequence_map);
        part1_answer += part1_length * password;
        part2_answer += part2_length * password;
    }

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
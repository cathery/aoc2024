use std::io::BufRead;

pub struct Point<N> {
    pub x: N,
    pub y: N,
}

// part 1
struct WordMatch {
    reverse: bool,
    start: Point<usize>,
    direction: Point<i8>,
    count: usize,
}

// part 2
struct CrossMatch {
    reverse_left: bool,
    reverse_right: bool,
    start: Point<usize>,
    failed: bool,
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let lines = std::io::BufReader::new(file).lines().flatten();

    let pattern1 = ['X', 'M', 'A', 'S'];
    let pattern2 = ['M', 'A', 'S'];

    let mut matches: Vec<WordMatch> = Vec::new();
    let mut new_matches: Vec<WordMatch> = Vec::new();

    let mut cross_matches: Vec<CrossMatch> = Vec::new();

    let mut finds = 0;
    let mut finds2 = 0;

    for (column, line) in lines.enumerate() {
        for (row, character) in line.char_indices() {

            /////////////////////
            //// PART 1
            /////////////////////

            // iterate over possible matches against the current letter
            for word_match in &mut matches {

                // ignore failed/completed matches
                if word_match.count < 1 {
                    continue;
                }

                if word_match.count == 1 {
                    // for the first letter, we want any adjacent letters
                    if row.abs_diff(word_match.start.x) > 1 || column.abs_diff(word_match.start.y) > 1 {
                        continue;
                    }
                }
                else {
                    // for 2+ letters we want a matching direction
                    if row != ((word_match.start.x as i32) + ((word_match.direction.x as i32) * (word_match.count as i32))) as usize
                    || column != ((word_match.start.y as i32) + ((word_match.direction.y as i32) * (word_match.count as i32))) as usize {
                        continue;
                    }
                }

                // we only want letters following the sequence
                if word_match.reverse && character != pattern1[pattern1.len() - word_match.count - 1]
                || !word_match.reverse && character != pattern1[word_match.count] {

                    if word_match.count > 1 {
                        // for 2+ letters, this means failure
                        word_match.count = 0;
                    }
                    continue;
                }

                // for first letter, any direction is fine
                if word_match.count == 1 {
                    // add a new match with a specified direction
                    new_matches.push(WordMatch {
                        reverse: word_match.reverse,
                        start: Point{x: word_match.start.x, y: word_match.start.y},
                        direction: Point {x: ((row as i32) - (word_match.start.x as i32)) as i8, y: ((column as i32) - (word_match.start.y as i32)) as i8},
                        count: word_match.count + 1
                    });

                    continue;
                }

                // if we reached this point, we have successfully matched the pattern
                if word_match.count == pattern1.len() - 1 {
                    // mark this match as completed
                    word_match.count = 0;
                    finds += 1;
                    continue;
                }
                else {
                    word_match.count += 1;
                }
            }

            matches.append(&mut new_matches);

            // add new possible matches
            if character == pattern1[0] || character == pattern1[pattern1.len() - 1] {
                matches.push(WordMatch {
                    reverse: character != pattern1[0],
                    start: Point{x: row, y: column},
                    direction: Point{x: 0, y: 0},
                    count: 1,
                });
            }

            /////////////////////
            //// PART 2
            /////////////////////

            // iterate over possible matches against the current letter
            for cross_match in &mut cross_matches {

                // ignore failed/completed matches
                if cross_match.failed {
                    continue;
                }

                // determine the flow of the right-to-left word
                if row == cross_match.start.x + pattern2.len() - 1
                && column == cross_match.start.y {
                    cross_match.reverse_right = character != pattern2[0];
                }

                // check if the current position matches the X pattern
                for count in 1..=pattern2.len() {

                    if column == cross_match.start.y + count - 1 {

                        // search for next left-to-right letter
                        if row == cross_match.start.x + count - 1 {
                            // if the character doesn't match, fail immediately
                            if cross_match.reverse_left && character != pattern2[pattern2.len() - count]
                            || !cross_match.reverse_left && character != pattern2[count - 1] {

                                cross_match.failed = true;
                                break;
                            }
                            else if count == pattern2.len() {
                                // this must be a success!
                                cross_match.failed = true;
                                finds2 += 1;
                                break;
                            }
                        }

                        // search for next right-to-left letter
                        if row == cross_match.start.x + pattern2.len() - count {
                            // if the character doesn't match, fail immediately
                            if cross_match.reverse_right && character != pattern2[pattern2.len() - count]
                            || !cross_match.reverse_right && character != pattern2[count - 1] {

                                cross_match.failed = true;
                                break;
                            }
                        }
                    }
                }
            }

            // add new possible matches
            if character == pattern2[0] || character == pattern2[pattern2.len() - 1] {
                cross_matches.push(CrossMatch {
                    reverse_left: character != pattern2[0],
                    reverse_right: false,
                    start: Point{x: row, y: column},
                    failed: false,
                });
            }
        }
    }

    println!("Part 1: {}", finds);
    println!("Part 2: {}", finds2);
}
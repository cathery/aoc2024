use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, usize};

const PRICE_CHANGES: usize = 2000;

type Secret = usize;
fn generate_secret(mut secret: Secret) -> Secret {
    const PRUNE: usize = 1 << 24;
    
    secret = secret ^ (secret << 6) % PRUNE;
    secret = secret ^ (secret >> 5) % PRUNE;
    secret = secret ^ (secret << 11) % PRUNE;
    
    return secret;
}

type Price = i8;
fn price(secret: Secret) -> Price {
    return (secret % 10) as Price;
}

type Sequence = u32;
fn push_sequence(sequence: &mut Sequence, change: Price) {
    *sequence = (*sequence << 8) + change as Sequence;
}

fn main() {
    let input = BufReader::new(File::open("input.txt").unwrap());

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    // Sequence -> buyer -> price at first occurence
    let mut sequences: HashMap<Sequence, HashMap<usize, Price>> = Default::default();

    for (buyer, line) in input.lines().enumerate() {
        let mut secret = line.unwrap().parse().unwrap();
        let mut old_price = price(secret);

        let mut sequence: Sequence = Default::default();
        for i in 1..=PRICE_CHANGES {
            secret = generate_secret(secret);

            let new_price = price(secret);

            // write the changes in price
            push_sequence(&mut sequence, new_price - old_price);

            // if there's at least 4 changes, start writing them down
            if i >= 4 {
                // write the price at the first occurence of this sequence to this buyer
                sequences.entry(sequence).or_default().entry(buyer).or_insert(new_price);
            }

            old_price = new_price;
        }

        part1_answer += secret;
    }

    for buyers in sequences.values() {
        let mut sum: usize = 0;
        for value in buyers.values() {
            sum += *value as usize;
        }
        part2_answer = part2_answer.max(sum);
    }

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
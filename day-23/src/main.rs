use std::{collections::{HashMap, HashSet}, fs::File, io::{BufReader, Read}};

type Computer = u16;

fn to_string(computer: Computer) -> String {
    let mut result = String::new();
    result.push((computer >> 8) as u8 as char);
    result.push(computer as u8 as char);
    return result;
}

fn main() {
    let mut input = BufReader::new(File::open("input.txt").unwrap());

    let mut computers: HashMap<Computer, HashSet<Computer>> = Default::default();

    let mut buf6: [u8; 6] = Default::default();

    // Read connections
    loop {
        let res = input.read_exact(&mut buf6);
        if res.is_err() || buf6[0] == b'\n' {
            break;
        }

        let first_computer:  Computer = ((buf6[0] as u16) << 8) + buf6[1] as u16;
        let second_computer: Computer = ((buf6[3] as u16) << 8) + buf6[4] as u16;

        computers.entry(first_computer).or_default().insert(second_computer);
        computers.entry(second_computer).or_default().insert(first_computer);
    }

    // Solve part 1
    let mut sets: HashSet<[Computer; 3]> = Default::default();
    for (computer, connections) in &computers {
        if (computer >> 8) as u8 == b't' {
            let connections_vec = Vec::from_iter(connections);
            for second in 0..connections_vec.len()-1 {
                for third in second+1..connections_vec.len() {
                    if computers[connections_vec[second]].contains(connections_vec[third]) {
                        let mut set = [*computer, *connections_vec[second], *connections_vec[third]];
                        set.sort();
                        sets.insert(set);
                    }
                }
            }
        }
    }
    let part1_answer = sets.len();


    // Solve part 2
    let mut largest_network: HashSet<_> = Default::default();
    for (computer, connections1) in &computers {
        for second in connections1 {
            let connections2 = &computers[second];
            let intersection: HashSet<Computer> = connections1.intersection(&connections2).copied().collect();
            let mut network = intersection.clone();
            network.insert(*computer);
            network.insert(*second);

            // make sure every computer in the intersection is connected to each other
            for third in &intersection {
                let mut network3 = computers[third].clone();
                network3.insert(*third);
                network = network.intersection(&network3).copied().collect();
            }

            if network.len() > largest_network.len() {
                largest_network = network;
            }
        }
    }

    let mut network_vec = Vec::from_iter(largest_network);
    network_vec.sort();
    let part2_answer = network_vec.iter().map(|x| to_string(*x)).collect::<Vec<_>>().join(",");

    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
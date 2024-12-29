use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader}};

fn run_program(mut wires: HashMap<String, bool>, mut gates: VecDeque<(String, (String, String, String))>) -> usize {
    while let Some((output, (wire1, gate, wire2))) = gates.pop_front() {
        if wires.contains_key(&wire1) && wires.contains_key(&wire2) {
            let input1 = *wires.get(&wire1).unwrap();
                let input2 = *wires.get(&wire2).unwrap();
                let value = match gate.as_str() {
                    "AND" => { input1 & input2 }
                    "OR"  => { input1 | input2 }
                    "XOR" => { input1 ^ input2 }
                    _ => panic!()
                };

                wires.insert(output, value);
        }
        else {
            gates.push_back((output, (wire1, gate, wire2)));
        }
    }

    let mut answer = 0;
    for i in 0..=45 {
        let wire = format!("z{:02}", i);
        let value = (*wires.get(&wire).unwrap_or(&false)) as usize;

        answer += value << i;
    }

    return answer;
}

fn main() {
    let mut input = BufReader::new(File::open("input.txt").unwrap());

    let mut buffer = String::new();

    let mut wires: HashMap<String, bool> = Default::default();
    let mut gates: HashMap<String, (String, String, String)> = Default::default();

    // Read wires
    loop {
        buffer.clear();
        let res = input.read_line(&mut buffer);
        if res.is_err() || buffer.starts_with('\n') {
            break;
        }

        let wire = &buffer[..3];
        let value = buffer.ends_with("1\n");
        wires.insert(wire.to_string(), value);
    }

    // Read gates
    for line in input.lines() {
        let var_name = line.unwrap();
        let oops: Vec<_> = var_name.split_whitespace().collect();
        let wire1 = oops[0];
        let gate = oops[1];
        let wire2 = oops[2];
        let output = oops[4];

        gates.insert(output.to_string(), (wire1.to_string(), gate.to_string(), wire2.to_string()));
    }

    // Solve part 2
    let mut wrong_outputs: Vec<String> = Default::default();
    for (output, (wire1, gate, wire2)) in &gates {
        if output.starts_with('z') {
            if output == "z00" || output == "z01" || output == "z45" {
                // unimplemented sorry
            }
            else if gate != "XOR" {
                // z outputs must come from a XOR gate
                wrong_outputs.push(output.clone());
            }
            else {
                // z must come from a XOR gate with one input from an OR gate and one input from an x XOR y gate
                const XY: &[char; 2] = &['x','y'];

                let (wire1_leftinput, wire1_gate, wire1_rightinput) = &gates[wire1];
                let (wire2_leftinput, wire2_gate, wire2_rightinput) = &gates[wire2];

                if wire1_gate != "OR" && (wire1_gate != "XOR" || !wire1_leftinput.starts_with(XY) || !wire1_rightinput.starts_with(XY)) {
                    wrong_outputs.push(wire1.clone());
                }

                if wire2_gate != "OR" && (wire2_gate != "XOR" || !wire2_leftinput.starts_with(XY) || !wire2_rightinput.starts_with(XY)) {
                    wrong_outputs.push(wire2.clone());
                }
            }
        }
        else if gate == "OR" {
            // both inputs of an OR gate must come from an AND gate
            let (_, wire1_gate, _) = &gates[wire1];
            let (_, wire2_gate, _) = &gates[wire2];

            if wire1_gate != "AND" {
                wrong_outputs.push(wire1.clone());
            }

            if wire2_gate != "AND" {
                wrong_outputs.push(wire2.clone());
            }
        }
        else {
            // unimplemented sorry
        }
    }
    wrong_outputs.sort();

    let part1_answer = run_program(wires, VecDeque::from_iter(gates));
    let part2_answer = wrong_outputs.join(",");


    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}
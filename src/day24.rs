use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt::format;
use advent_tools::fetch_data;
use itertools::Itertools;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/24/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let mut test = vec!["wgb", "wbw", "jct", "z39", "z09", "gwh", "z21", "rcb"];
    test.sort();
    println!("{:?}", test.join(","));

    let start = Instant::now();
    let gates = get_gates(&data);
    let linked_wires = link_wires_to_gates(&gates);

    let initial_wires = set_initial_wires(0, 0);
    let final_wires = evaluate(&initial_wires, &linked_wires);

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", 0, duration);

    Ok(())
}

fn get_result(wires: &HashMap<String, usize>) -> i64 {
    let mut bits = vec![];
    for i in 0..100 {
        let wire = format!("z{:02}", i);
        if let Some(&value) = wires.get(&wire) {
            bits.push(value);
        } else {
            break
        }
    }

    let mut res = 0;
    for (i, &bit) in bits.iter().enumerate() {
        res += (bit as i64) * 2_i64.pow(i as u32);
    }
    res
}
fn get_result_binary(wires: &HashMap<String, usize>) -> String {
    let mut bits = vec![];
    for i in 0..100 {
        let wire = format!("z{:02}", i);
        if let Some(&value) = wires.get(&wire) {
            bits.push(value);
        } else {
            break
        }
    }

    bits.iter().map(|b| format!("{b}")).join("")
}

fn evaluate(initial_wires: &HashMap<String, usize>, linked_wires: &HashMap<String, Vec<&Gate>>) -> HashMap<String, usize> {
    let mut final_wires = initial_wires.clone();
    let mut evaluated_gates: Vec<&Gate> = vec![];
    let mut tasks: VecDeque<&Gate> = VecDeque::new();
    for i in 0..45 {
        let wire_x = format!("x{:02}", i);
        let gates = linked_wires.get(&wire_x).unwrap();
        for &gate in gates {
            tasks.push_back(gate);
        }
        let wire_y = format!("y{:02}", i);
        let gates = linked_wires.get(&wire_y).unwrap();
        for &gate in gates {
            tasks.push_back(gate);
        }
    }
    while let Some((gate)) = tasks.pop_front() {
        if evaluated_gates.contains(&gate) {
            continue;
        }
        if let Some(a_value) = final_wires.get(&gate.a_wire) {
            if let Some(b_value) = final_wires.get(&gate.b_wire) {
                let output_val = match gate.operation.as_str() {
                    "AND" => a_value & b_value,
                    "OR" => a_value | b_value,
                    "XOR" => a_value ^ b_value,
                    _ => panic!("Unknown operation: {}", gate.operation),
                };
                final_wires.insert(gate.output_wire.clone(), output_val);
                evaluated_gates.push(gate);
                print_gate(gate);
                if let Some(gates) = linked_wires.get(&gate.output_wire) {
                    // sort gates by gate.operation
                    let mut gates = gates.iter().map(|&g| g).collect::<Vec<&Gate>>();
                    gates.sort_by(|a, b| a.operation.cmp(&b.operation));
                    for gate in gates {
                        tasks.push_front(gate);
                    }
                }
            }
        }
    }

    final_wires
}

fn link_wires_to_gates(gates: &Vec<Gate>) -> HashMap<String, Vec<&Gate>> {
    let mut linked_wires = HashMap::new();
    for gate in gates {
        linked_wires
            .entry(gate.a_wire.clone())
            .or_insert(vec![])
            .push(gate);
        linked_wires
            .entry(gate.b_wire.clone())
            .or_insert(vec![])
            .push(gate);
    }
    linked_wires
}

fn get_gates(data: &Vec<String>) -> Vec<Gate> {
    data.iter()
        .filter(|line| line.contains("->"))
        .map(|line| {
            let output_wire = line.split(" -> ").last().unwrap().to_string();
            let input_str = line.split(" -> ").nth(0).unwrap();
            let a_wire = input_str.split_whitespace().nth(0).unwrap().to_string();
            let operation = input_str.split_whitespace().nth(1).unwrap().to_string();
            let b_wire = input_str.split_whitespace().nth(2).unwrap().to_string();
            Gate{a_wire, b_wire, operation, output_wire}
        })
        .collect()
}

fn set_initial_wires(x: i64, y: i64) -> HashMap<String, usize> {
    let mut wires = HashMap::new();
    for i in 0..45 {
        let bit = (x >> i) & 1;
        wires.insert(format!("x{:02}", i), bit as usize);
    }
    for i in 0..45 {
        let bit = (y >> i) & 1;
        wires.insert(format!("y{:02}", i), bit as usize);
    }
    wires
}

fn get_initial_wires(data: &Vec<String>) -> HashMap<String, usize> {
    let mut wires = HashMap::new();
    for line in data {
        if line.contains(": ") {
            let wire = line.split(": ").nth(0).unwrap().to_string();
            let wire_value: usize = line.split(": ").nth(1).unwrap().parse().unwrap();
            wires.insert(wire, wire_value);
        }
    }
    wires
}

fn test_data() -> Vec<String> {
    // r"  x00: 1
    //     x01: 1
    //     x02: 1
    //     y00: 0
    //     y01: 1
    //     y02: 0
    //
    //     x00 AND y00 -> z00
    //     x01 XOR y01 -> z01
    //     x02 OR y02 -> z02"
    r"  x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
struct Gate {
    a_wire: String,
    b_wire: String,
    operation: String,
    output_wire: String,
}

fn print_gate(gate: &Gate) {
    println!("{:?} {} {:?} -> {:?}", gate.a_wire, gate.operation, gate.b_wire, gate.output_wire);
}
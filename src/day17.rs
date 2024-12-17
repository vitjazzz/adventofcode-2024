use std::error::Error;
use advent_tools::fetch_data;
use tokio::time::Instant;

pub async fn execute() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/17/input";
    let data = fetch_data(url).await?;
    // let data = test_data();

    let start = Instant::now();
    let program = get_program(&data);

    let mut i = 0;
    let mut program_pointer = 15;
    while program_pointer >= 0 {
        let mut computer = get_computer(&data);
        computer.register_a = i;
        if let Ok(_) = execute_program(&program, &mut computer) {
            println!("Final Result: {}", i);
            break;
        }
        if computer.output.len() > 16 {
            break;
        }
        if same_last(&computer.output, &program) {
            program_pointer -= 1;
            i = i << 3;
        } else {
            i += 1;
        }
    }

    let duration = start.elapsed();
    println!("Result: {}, Execution time: {:?}", 0, duration);

    Ok(())
}

fn same_last(output: &Vec<i64>, program: &Vec<i64>) -> bool {
    for i in 0..output.len() {
        if output[i] != program[program.len() - output.len() + i] {
            return false;
        }
    }
    true
}

fn execute_program(program: &Vec<i64>, mut computer: &mut Computer) -> Result<(), Box<dyn Error>> {
    while computer.instruction_pointer < (program.len() - 1) as i64 {
        let operation_type = program[computer.instruction_pointer as usize];
        let operand = program[computer.instruction_pointer as usize + 1];
        match operation_type {
            0 => adv(&mut computer, operand),
            1 => bxl(&mut computer, operand),
            2 => bst(&mut computer, operand),
            3 => jnz(&mut computer, operand),
            4 => bxc(&mut computer, operand),
            5 => out(&mut computer, operand),
            6 => bdv(&mut computer, operand),
            7 => cdv(&mut computer, operand),
            _ => {}
        }
        //  2,4,1,3,7,5,4,7,0,3,1,5,5,5,3,0
        //  bst 4 (B=A % 8) - bxl 3 (B=B ^ 3) - cdv 5 (C=A / 2^B) - bxc 7 (B=B ^ C) - adv 3 (A=A / 2^3) - bxl 5 (B=B ^ 5) - out 5 (B % 8) - jnz 0
        //  bst 4 (B=A % 8) - bxl 3 (B=B ^ 3) - cdv 5 (C=A / 2^B) - bxc 7 (B=B ^ C) - adv 3 (A=A / 2^3) - bxl 5 (B=0bxxxx11) - out 5 (B=0bxxxx10) - jnz 0
        if computer.output.len() > 0 && *computer.output.last().unwrap() != program[computer.output.len() - 1]
            || computer.output.len() > program.len() {
            // break;
        }
    }
    if computer.output == *program {
        Ok(())
    } else {
        Err("Output does not match program".into())
    }
}

fn get_program(data: &Vec<String>) -> Vec<i64> {
    data.last().unwrap().split_whitespace().last().unwrap().split(',').map(|s| s.parse().unwrap()).collect()
}

fn get_computer(data: &Vec<String>) -> Computer {
    Computer {
        register_a: data[0].split_whitespace().last().unwrap().parse().unwrap(),
        register_b: data[1].split_whitespace().last().unwrap().parse().unwrap(),
        register_c: data[2].split_whitespace().last().unwrap().parse().unwrap(),
        instruction_pointer: 0,
        output: vec![],
    }
}

fn adv(computer: &mut Computer, operand: i64) {
    let numerator = computer.register_a;
    let operand = computer.get_combo_operand(operand);
    let denominator = 2i64.pow(operand as u32);
    computer.register_a = numerator / denominator;
    computer.instruction_pointer += 2;
}

fn bxl(computer: &mut Computer, operand: i64) {
    computer.register_b = computer.register_b ^ operand;
    computer.instruction_pointer += 2;
}

fn bst(computer: &mut Computer, operand: i64) {
    let operand = computer.get_combo_operand(operand);
    computer.register_b = operand % 8;
    computer.instruction_pointer += 2;
}

fn jnz(computer: &mut Computer, operand: i64) {
    if computer.register_a == 0 {
        computer.instruction_pointer += 2;
        return;
    }
    computer.instruction_pointer = operand;
}

fn bxc(computer: &mut Computer, _: i64) {
    computer.register_b = computer.register_b ^ computer.register_c;
    computer.instruction_pointer += 2;
}

fn out(computer: &mut Computer, operand: i64) {
    let operand = computer.get_combo_operand(operand);
    let output = operand % 8;
    computer.output.push(output);
    computer.instruction_pointer += 2;
}

fn bdv(computer: &mut Computer, operand: i64) {
    let numerator = computer.register_a;
    let operand = computer.get_combo_operand(operand);
    let denominator = 2i64.pow(operand as u32);
    computer.register_b = numerator / denominator;
    computer.instruction_pointer += 2;
}

fn cdv(computer: &mut Computer, operand: i64) {
    let numerator = computer.register_a;
    let operand = computer.get_combo_operand(operand);
    let denominator = 2i64.pow(operand as u32);
    computer.register_c = numerator / denominator;
    computer.instruction_pointer += 2;
}

fn test_data() -> Vec<String> {
    r"  Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0"
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: i64,
    output: Vec<i64>,
}

impl Computer {
    fn get_combo_operand(&self, operand: i64) -> i64 {
        match operand {
            o if o <= 3 => o,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand"),
        }
    }
}
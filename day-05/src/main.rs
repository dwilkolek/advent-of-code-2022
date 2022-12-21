#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let entries: Vec<String> = lines
            .map(|line| match line {
                Ok(line) => line,
                _ => panic!("Wtf"),
            })
            .collect();

        let split_at = find_split_at(&entries);
        let (state, instructions) = entries.split_at(split_at);

        let mut yard = convert_state_to_yard(state.to_vec());

        let operations = convert_operations(instructions.to_vec());

        // yard.apply_operations(&operations);
        yard.apply_operations_9001(&operations);

        yard.top_boxes()
    }
}
fn convert_operations(instructions: Vec<String>) -> Vec<Operation> {
    let operations: Vec<Operation> = instructions
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| Operation::from_entry(&line))
        .collect();
    // println!("Operations: {:?}", operations);
    return operations;
}
fn convert_state_to_yard(state: Vec<String>) -> Yard {
    let mut state = state.to_vec();
    // let mut stack_mapping = HashMap::new();
    let stack_positions_regex = Regex::new("([0-9]+)").unwrap();

    let stack_ids = state.pop().unwrap();
    let stack_count = stack_positions_regex
        .captures_iter(stack_ids.trim())
        .count();

    let mut yard = Yard::init(stack_count);

    //create yard from bottom stack
    for line in state.into_iter().rev() {
        read_stack_line(&line, &mut yard);
    }

    // println!("Yard: {:?}", yard);
    return yard;
}

fn read_stack_line(stack_row: &str, yard: &mut Yard) {
    let box_id_reg = Regex::new("\\[(.)\\]").unwrap();

    //Map String to chars
    let stack_row = stack_row
        .chars()
        .collect::<Vec<char>>()
        .chunks(4) // `[A] ``[B] ` <- each chunk takes 4 characters
        .map(|c| c.iter().collect::<String>().trim().to_owned()) //Trim to `[A] ` to `[A]` and return to chars
        .collect::<Vec<String>>();
    for (stack_index, c) in stack_row.into_iter().enumerate() {
        //If box exists then place on stack
        match box_id_reg.captures(&c) {
            Some(capture) => yard.add_to_stack(
                stack_index,
                capture.get(1).unwrap().as_str().chars().nth(0).unwrap(),
            ),
            None => (),
        }
    }
}

fn find_split_at(entries: &Vec<String>) -> usize {
    for line in entries.clone().into_iter().enumerate() {
        if line.1 == "" {
            return line.0;
        }
    }
    panic!("Failed to find splitting point")
}

#[derive(Debug)]
struct Yard {
    stacks: Vec<Vec<char>>,
}

impl Yard {
    fn init(stack_count: usize) -> Yard {
        let mut stacks = Vec::with_capacity(stack_count);
        println!("Creating Yard with {} stacks", stack_count);
        for _ in 0..stack_count {
            stacks.push(Vec::with_capacity(10000));
        }
        Yard { stacks: stacks }
    }

    fn add_to_stack(&mut self, stack: usize, box_id: char) {
        self.stacks[stack].push(box_id);
    }

    fn apply_operations(&mut self, operations: &Vec<Operation>) {
        operations.into_iter().for_each(|op| {
            for _ in 0..op.count {
                let to_move = self.stacks[op.from - 1].pop().unwrap();
                self.stacks[op.to - 1].push(to_move)
            }
        })
    }

    fn apply_operations_9001(&mut self, operations: &Vec<Operation>) {
        operations.into_iter().for_each(|op| {
            let mut chunk = vec![];
            for _ in 0..op.count {
                chunk.push(self.stacks[op.from - 1].pop().unwrap());
            }
            chunk.reverse();
            self.stacks[op.to - 1].append(&mut chunk);
        })
    }

    fn top_boxes(&self) {
        let res: String = self
            .stacks
            .clone()
            .into_iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect();
        println!("Top boxes {}", res);
    }
}

#[derive(Debug)]
struct Operation {
    // move 1 from 2 to 1
    count: usize,
    from: usize,
    to: usize,
}

impl Operation {
    fn from_entry(line: &str) -> Operation {
        let re = Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        for cap in re.captures(&line).into_iter() {
            return Operation {
                count: cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                from: cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                to: cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            };
        }
        panic!("Failed to create Operation")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use regex::Regex;
use std::collections::HashMap;
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

        let mut yard = convert_state_to_yard(state);

        let operations = convert_operations(instructions);

        // yard.apply_operations(&operations);
        yard.apply_operations_9001(&operations);

        yard.top_boxes()
    }
}
fn convert_operations(instructions: &[String]) -> Vec<Operation> {
    let operations: Vec<Operation> = instructions
        .to_vec()
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| Operation::from_entry(l))
        .collect();
    // println!("Operations: {:?}", operations);
    return operations;
}
fn convert_state_to_yard(state: &[String]) -> Yard {
    let mut state = state.to_vec();
    // let mut stack_mapping = HashMap::new();
    let stack_positions_regex = Regex::new("([0-9]+)").unwrap();

    let stack_ids = state.pop().unwrap();
    let stack_count = stack_positions_regex
        .captures_iter(stack_ids.trim())
        .count();

    let mut yard = Yard::init(stack_count);

    //create yard from bottom stack
    for l in state.into_iter().rev() {
        read_stack_line(l, &mut yard);
    }

    // println!("Yard: {:?}", yard);
    return yard;
}

fn read_stack_line(stack_row: String, yard: &mut Yard) {
    let box_id_reg = Regex::new("\\[(.)\\]").unwrap();
    //Map String to chars
    let stack_row = stack_row.chars().collect::<Vec<char>>();

    // `[A] ``[B] ` <- each chunk takes 4 characters
    let stack_row = stack_row.chunks(4).collect::<Vec<&[char]>>().clone();

    //Map chars to chars
    let stack_row: Vec<String> = stack_row
        .into_iter()
        .map(|c| String::from_iter(c))
        .collect();

    //Trim to `[A] ` to `[A]`
    let stack_row: Vec<String> = stack_row
        .into_iter()
        .map(|c| c.trim().to_string())
        .collect();

    for (ic, c) in stack_row.into_iter().enumerate() {
        //is there a box?
        if box_id_reg.captures(&c).is_some() {
            for (i, captures) in box_id_reg.captures_iter(&c).enumerate() {
                let box_id = captures.get(1).unwrap().as_str().chars().nth(0).unwrap();
                yard.add_to_stack(ic, box_id) // put box on stack
            }
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
        for i in 0..stack_count {
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
    fn from_entry(line: String) -> Operation {
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

// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;

const DEBUG: bool = false;

#[derive(Debug)]
enum Op {
    ADDX(isize),
    NOOP,
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    requred_cycles: usize,
}

impl Instruction {
    fn execute(&mut self, x: &mut isize) -> bool {
        self.requred_cycles = self.requred_cycles - 1;
        match &self.op {
            Op::ADDX(increment) => {
                if self.requred_cycles == 0 {
                    *x += increment;
                    true
                } else {
                    false
                }
            }
            Op::NOOP => {
                if self.requred_cycles == 0 {
                    true
                } else {
                    false
                }
            }
        }
    }

    fn addx(increment: isize) -> Instruction {
        Instruction {
            op: Op::ADDX(increment),
            requred_cycles: 2,
        }
    }

    fn noop() -> Instruction {
        Instruction {
            op: Op::NOOP,
            requred_cycles: 1,
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
    cycle: usize,
    x: isize,
}

impl Program {
    fn execute(&mut self) -> bool {
        if self.instructions.len() == 0 {
            //noop
            return false;
        }
        let is_done = self.instructions[0].execute(&mut self.x);
        if is_done {
            self.instructions.remove(0);
        }
        true
    }

    fn next_cycle(&mut self) {
        self.cycle += 1;
    }
}

fn main() {
    let mut instructions = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for cmd in input.into_iter() {
            let parts: Vec<&str> = cmd.split(" ").into_iter().collect();
            match parts[0] {
                "addx" => {
                    instructions.push(Instruction::addx(parts[1].to_owned().parse().unwrap()))
                }
                "noop" => instructions.push(Instruction::noop()),
                _ => unreachable!(),
            }
        }
    }

    let mut program = Program {
        cycle: 1,
        instructions,
        x: 1,
    };
    let mut checkpoints: HashMap<usize, isize> = HashMap::new();
    let screen_px_count = 240;
    loop {
        sprite(&program);
        draw(&program);
        let more_instructions = program.execute();

        program.next_cycle();

        if program.cycle == 20 || (program.cycle > 20 && (program.cycle - 20) % 40 == 0) {
            checkpoints.insert(program.cycle, program.x);
        }
        if program.cycle == screen_px_count + 1 {
            break;
        }
        if !more_instructions {
            break;
        }
    }

    let strength: isize = checkpoints
        .into_iter()
        .map(|entry| entry.0 as isize * entry.1)
        .sum();

    println!("Signal strength: {}", strength)
}

fn sprite(program: &Program) {
    if DEBUG {
        print!("Sprite position {} {}: ", program.cycle, program.x);
        for i in 0..39 {
            if i == program.x || i == program.x - 1 || i == program.x + 1 {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("")
    }
}

fn draw(program: &Program) {
    let crt_pos: isize = ((program.cycle - 1) % 40) as isize;
    let new_line = crt_pos == 39;
    let sprite_visible =
        crt_pos == program.x || crt_pos == program.x - 1 || crt_pos == program.x + 1;

    if new_line {
        println!("{}", if sprite_visible { "#" } else { "." })
    } else {
        print!("{}", if sprite_visible { "#" } else { "." })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

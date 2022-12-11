#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

#[derive(Debug)]
enum Operation {
    ADD(What, What),
    MULTIPLY(What, What),
}

impl Operation {
    fn from_str(str: &str, first: What, second: What) -> Operation {
        match str {
            "+" => Operation::ADD(first, second),
            "*" => Operation::MULTIPLY(first, second),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum What {
    OLD,
    VALUE(usize),
}

impl What {
    fn from_str(str: &str) -> What {
        match str {
            "old" => What::OLD,
            value => What::VALUE(value.parse().unwrap()),
        }
    }
}
#[derive(Debug)]
struct Item {
    insepcted: usize,
    worry_level: usize,
}

impl Item {
    fn apply_op(&mut self, op: &Operation, limiter: usize) {
        self.insepcted = self.insepcted + 1;
        match op {
            Operation::ADD(a, b) => {
                self.worry_level =
                    what_to_val(a, &self.worry_level) + what_to_val(b, &self.worry_level)
            }
            Operation::MULTIPLY(a, b) => {
                self.worry_level =
                    what_to_val(a, &self.worry_level) * what_to_val(b, &self.worry_level)
            }
        }
        self.worry_level = self.worry_level % limiter;
    }

    fn bored(&mut self) {
        self.worry_level = self.worry_level / 3
    }
}

fn what_to_val(what: &What, worry_level: &usize) -> usize {
    match what {
        What::OLD => worry_level.clone(),
        What::VALUE(v) => v.clone(),
    }
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    test: usize,
    on_true: usize,
    on_false: usize,
    items: Vec<Item>,
}

fn main() {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut tests: Vec<usize> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for monkey in input.chunks(7).into_iter() {
            let monkey_id = monkey[0].replace("Monkey ", "");
            let monkey_id = monkey_id.replace(":", "").parse::<usize>().unwrap();

            let items_reg = Regex::new("([0-9]+)[, ]*").unwrap();
            let items_cap = items_reg.captures_iter(monkey[1].as_str());
            let mut items: Vec<Item> = items_cap
                .map(|item| Item {
                    insepcted: 0,
                    worry_level: item
                        .get(1)
                        .unwrap()
                        .as_str()
                        .to_owned()
                        .parse::<usize>()
                        .unwrap(),
                })
                .collect();
            println!("{:?}", items.reverse());

            let operation_reg = Regex::new("(old|[0-9]+) ([*+]) (old|[0-9]+)").unwrap();
            let operation_cap = operation_reg.captures(monkey[2].as_str()).unwrap();
            let first = What::from_str(operation_cap.get(1).unwrap().as_str());
            let second = What::from_str(operation_cap.get(3).unwrap().as_str());
            let operation =
                Operation::from_str(operation_cap.get(2).unwrap().as_str(), first, second);
            println!("{:?}", operation);

            let test = monkey[3]
                .replace("  Test: divisible by ", "")
                .parse::<usize>()
                .unwrap();
            tests.push(test);
            println!("Devide by {:?}", test);

            let on_true = monkey[4]
                .replace("    If true: throw to monkey ", "")
                .parse::<usize>()
                .unwrap();
            println!("On true {:?}", on_true);
            let on_false = monkey[5]
                .replace("    If false: throw to monkey ", "")
                .parse::<usize>()
                .unwrap();
            println!("On false {:?}", on_false);

            monkeys.insert(
                monkey_id,
                Monkey {
                    items,
                    operation,
                    test,
                    on_true,
                    on_false,
                },
            );
        }
    }

    let monkey_len = monkeys.len();
    let mut inspection_log: HashMap<usize, usize> = HashMap::new();
    let mut common: usize = 1;
    for c in tests.clone().into_iter() {
        common = common * c;
    }

    for _ in 0..10000 {
        for m in 0..monkey_len {
            let t = monkeys[m].on_true.clone();
            let f = monkeys[m].on_false.clone();

            while let Some(mut item) = monkeys[m].items.pop() {
                item.apply_op(&monkeys[m].operation, common);

                if let Some(count) = inspection_log.get(&m) {
                    inspection_log.insert(m, count + 1);
                } else {
                    inspection_log.insert(m, 1);
                };

                // item.bored();
                if item.worry_level % monkeys[m].test == 0 {
                    monkeys[t].items.push(item);
                } else {
                    monkeys[f].items.push(item);
                }
            }
        }
    }
    let mut values: Vec<&usize> = inspection_log.values().map(|v| v).collect();
    values.sort();
    values.reverse();

    println!("{:?}", values[0] * values[1]);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

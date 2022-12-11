// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

// (?m)Monkey (?P<monkey>[0-9]+):\n\s+.*Starting items: (?P<items>[0-9]+)[, ]+([0-9]+)\n\s+Operation: new = (?P<operation>.*)\n\s+Test: divisible by (?P<test>.*)\n\s+If true: throw to monkey (?P<on_true>.*)\n\s+If false: throw to monkey (?P<on_false>.*)

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

const DEBUG: bool = false;

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
    VALUE(isize),
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
    worry_level: isize,
}

impl Item {
    fn apply_op(&mut self, op: &Operation) {
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
    }

    fn bored(&mut self) {
        self.worry_level = self.worry_level / 3
    }
}

fn what_to_val(what: &What, worry_level: &isize) -> isize {
    match what {
        What::OLD => worry_level.clone(),
        What::VALUE(v) => v.clone(),
    }
}

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    test: isize,
    on_true: usize,
    on_false: usize,
    items: Vec<Item>,
}

fn main() {
    let mut monkeys: Vec<Monkey> = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for monkey in input.chunks(7).into_iter() {
            let monkey_id = monkey[0].replace("Monkey ", "");
            let monkey_id = monkey_id.replace(":", "").parse::<usize>().unwrap();
            println!("{}", monkey_id);

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
                        .parse::<isize>()
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
                .parse::<isize>()
                .unwrap();

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

    for round in 0..20 {
        for m in 0..monkey_len {
            let t = monkeys[m].on_true.clone();
            let f = monkeys[m].on_false.clone();

            while let Some(mut item) = monkeys[m].items.pop() {
                item.apply_op(&monkeys[m].operation);
                // inspection_log.get_mut(&m).and_then(|v| {
                //     match v {
                //         Some(v) => v = v+1,
                //     }
                // })
                if let Some(count) = inspection_log.get(&m) {
                    inspection_log.insert(m, count + 1);
                } else {
                    inspection_log.insert(m, 1);
                };

                item.bored();
                if item.worry_level % monkeys[m].test == 0 {
                    monkeys[t].items.push(item);
                } else {
                    monkeys[f].items.push(item);
                }
            }
        }

        for m in 0..monkey_len {
            println!("{} Monkey {}: {:?}", round, m, monkeys[m].items);
        }

        println!("{:?}", inspection_log);

        let mut values: Vec<&usize> = inspection_log.values().map(|v| v).collect();
        values.sort();
        values.reverse();

        println!("{:?}", values[0] * values[1]);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

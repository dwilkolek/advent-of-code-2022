// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Node {
    Value(f64),
    Divide(String, String),
    Multiply(String, String),
    Subtract(String, String),
    Add(String, String),
    Compare(String, String),
    Guess,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut monkeys: HashMap<String, f64> = HashMap::new();
    if let Ok(lines) = read_lines("input.txt") {
        let mut input: VecDeque<String> = lines.into_iter().map(|l| l.unwrap()).collect();

        while let Some(entry) = input.pop_front() {
            let p: Vec<&str> = entry.split(": ").into_iter().collect();
            match p[1].parse::<f64>() {
                Ok(value) => {
                    monkeys.insert(p[0].to_string(), value);
                }
                Err(_) => {
                    let equasion: Vec<&str> = p[1].split(" ").into_iter().collect();

                    match (monkeys.get(equasion[0]), monkeys.get(equasion[2])) {
                        (Some(a), Some(b)) => {
                            if p[0] == "root" {}
                            match equasion[1] {
                                "/" => monkeys.insert(p[0].to_owned(), a / b),
                                "*" => monkeys.insert(p[0].to_owned(), a * b),
                                "+" => monkeys.insert(p[0].to_owned(), a + b),
                                "-" => monkeys.insert(p[0].to_owned(), a - b),
                                _ => unreachable!(),
                            };
                        }
                        _ => {
                            input.push_back(entry);
                        }
                    };
                }
            }
        }

        println!("Part 1: {}", monkeys.get("root").unwrap())
    }
}

fn part2() {
    let mut operations: HashMap<String, Node> = HashMap::new();
    if let Ok(lines) = read_lines("input.txt") {
        let mut input: VecDeque<String> = lines.into_iter().map(|l| l.unwrap()).collect();

        while let Some(entry) = input.pop_front() {
            let p: Vec<&str> = entry.split(": ").into_iter().collect();

            match p[0] {
                "humn" => {
                    operations.insert("humn".to_owned(), Node::Guess);
                }
                _ => match p[1].parse::<f64>() {
                    Ok(value) => {
                        operations.insert(p[0].to_string(), Node::Value(value));
                    }
                    Err(_) => {
                        let equasion: Vec<&str> = p[1].split(" ").into_iter().collect();
                        match p[0] {
                            "root" => {
                                operations.insert(
                                    "root".to_owned(),
                                    Node::Compare(equasion[0].to_owned(), equasion[2].to_owned()),
                                );
                            }
                            _ => {
                                let op = match equasion[1] {
                                    "/" => {
                                        Node::Divide(equasion[0].to_owned(), equasion[2].to_owned())
                                    }

                                    "*" => Node::Multiply(
                                        equasion[0].to_owned(),
                                        equasion[2].to_owned(),
                                    ),

                                    "+" => {
                                        Node::Add(equasion[0].to_owned(), equasion[2].to_owned())
                                    }

                                    "-" => Node::Subtract(
                                        equasion[0].to_owned(),
                                        equasion[2].to_owned(),
                                    ),

                                    _ => unreachable!(),
                                };
                                operations.insert(p[0].to_string(), op);
                            }
                        };
                    }
                },
            }
        }
    }

    let mut optimized_nodes = HashMap::new();
    let mut cached_results = HashMap::new();
    replace_operations_with_values(
        "root",
        &operations,
        &mut optimized_nodes,
        &mut cached_results,
    );

    println!(
        "Part 2 Expression: {}",
        stringify(operations.get("root").unwrap(), &optimized_nodes)
    );
    println!("Part 2:  {:.4}", unwind(&optimized_nodes, &cached_results))
}

fn unwind(nodes: &HashMap<String, Node>, cached_results: &HashMap<String, f64>) -> f64 {
    let mut deq = VecDeque::new();
    deq.push_back(nodes.get("root").unwrap());
    let mut target = 0.0;
    while let Some(node) = deq.pop_front() {
        if let Node::Compare(a, b) = node {
            if let Some(v) = cached_results.get(a) {
                target = *v;
                deq.push_front(nodes.get(b).unwrap());
                continue;
            }

            if let Some(v) = cached_results.get(b) {
                target = -1.0 * *v;
                deq.push_front(nodes.get(a).unwrap());
                continue;
            }
        }

        if let Node::Divide(a, b) = node {
            if let Some(v) = cached_results.get(a) {
                target /= *v;
                deq.push_front(nodes.get(b).unwrap());
                continue;
            }

            if let Some(v) = cached_results.get(b) {
                target *= *v;
                deq.push_front(nodes.get(a).unwrap());
                continue;
            }
        }

        if let Node::Add(a, b) = node {
            if let Some(v) = cached_results.get(a) {
                target -= *v;
                deq.push_front(nodes.get(b).unwrap());
                continue;
            }

            if let Some(v) = cached_results.get(b) {
                target -= *v;
                deq.push_front(nodes.get(a).unwrap());
                continue;
            }
        }

        if let Node::Subtract(a, b) = node {
            if let Some(v) = cached_results.get(a) {
                target = target + v;
                deq.push_front(nodes.get(b).unwrap());
                continue;
            }

            if let Some(v) = cached_results.get(b) {
                target += *v;
                deq.push_front(nodes.get(a).unwrap());
                continue;
            }
        }

        if let Node::Multiply(a, b) = node {
            if let Some(v) = cached_results.get(a) {
                target /= *v;
                deq.push_front(nodes.get(b).unwrap());
                continue;
            }

            if let Some(v) = cached_results.get(b) {
                target /= *v;
                deq.push_front(nodes.get(a).unwrap());
                continue;
            }
        }
    }

    target
}

fn replace_operations_with_values(
    node_id: &str,
    nodes: &HashMap<String, Node>,
    optimized: &mut HashMap<String, Node>,
    cached_results: &mut HashMap<String, f64>,
) {
    let node = nodes.get(node_id).unwrap();
    match node {
        Node::Guess => {
            optimized.insert(node_id.to_owned(), Node::Guess);
        }
        Node::Compare(a, b) => {
            replace_operations_with_values(a, nodes, optimized, cached_results);
            replace_operations_with_values(b, nodes, optimized, cached_results);
            optimized.insert(node_id.to_owned(), Node::Compare(a.clone(), b.clone()));
        }
        Node::Value(v) => {
            cached_results.insert(node_id.to_owned(), *v);
            optimized.insert(node_id.to_owned(), Node::Value(*v));
        }
        Node::Divide(a, b) => {
            replace_operations_with_values(a, nodes, optimized, cached_results);
            replace_operations_with_values(b, nodes, optimized, cached_results);
            match (cached_results.get(a), cached_results.get(b)) {
                (Some(av), Some(bv)) => {
                    let calculated = av / bv;
                    cached_results.insert(node_id.to_owned(), calculated);
                    optimized.insert(node_id.to_owned(), Node::Value(calculated));
                }
                _ => {
                    optimized.insert(node_id.to_owned(), Node::Divide(a.clone(), b.clone()));
                }
            }
        }
        Node::Multiply(a, b) => {
            replace_operations_with_values(a, nodes, optimized, cached_results);
            replace_operations_with_values(b, nodes, optimized, cached_results);
            match (cached_results.get(a), cached_results.get(b)) {
                (Some(av), Some(bv)) => {
                    let calculated = av * bv;
                    cached_results.insert(node_id.to_owned(), calculated);
                    optimized.insert(node_id.to_owned(), Node::Value(calculated));
                }
                _ => {
                    optimized.insert(node_id.to_owned(), Node::Multiply(a.clone(), b.clone()));
                }
            }
        }
        Node::Subtract(a, b) => {
            replace_operations_with_values(a, nodes, optimized, cached_results);
            replace_operations_with_values(b, nodes, optimized, cached_results);
            match (cached_results.get(a), cached_results.get(b)) {
                (Some(av), Some(bv)) => {
                    let calculated = av - bv;
                    cached_results.insert(node_id.to_owned(), calculated);
                    optimized.insert(node_id.to_owned(), Node::Value(calculated));
                }
                _ => {
                    optimized.insert(node_id.to_owned(), Node::Subtract(a.clone(), b.clone()));
                }
            }
        }
        Node::Add(a, b) => {
            replace_operations_with_values(a, nodes, optimized, cached_results);
            replace_operations_with_values(b, nodes, optimized, cached_results);
            match (cached_results.get(a), cached_results.get(b)) {
                (Some(av), Some(bv)) => {
                    let calculated = av + bv;
                    cached_results.insert(node_id.to_owned(), calculated);
                    optimized.insert(node_id.to_owned(), Node::Value(calculated));
                }
                _ => {
                    optimized.insert(node_id.to_owned(), Node::Add(a.clone(), b.clone()));
                }
            }
        }
    }
}

fn stringify(node: &Node, nodes: &HashMap<String, Node>) -> String {
    match node {
        Node::Value(v) => format!("{}", v),
        Node::Divide(a, b) => format!(
            "({}/{})",
            stringify(nodes.get(a).unwrap(), nodes),
            stringify(nodes.get(b).unwrap(), nodes)
        ),
        Node::Multiply(a, b) => format!(
            "({}*{})",
            stringify(nodes.get(a).unwrap(), nodes),
            stringify(nodes.get(b).unwrap(), nodes)
        ),
        Node::Subtract(a, b) => format!(
            "({}-{})",
            stringify(nodes.get(a).unwrap(), nodes),
            stringify(nodes.get(b).unwrap(), nodes)
        ),
        Node::Add(a, b) => format!(
            "({}+{})",
            stringify(nodes.get(a).unwrap(), nodes),
            stringify(nodes.get(b).unwrap(), nodes)
        ),
        Node::Compare(a, b) => format!(
            "{} == {}",
            stringify(nodes.get(a).unwrap(), nodes),
            stringify(nodes.get(b).unwrap(), nodes)
        ),
        Node::Guess => " HUMN ".to_owned(),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

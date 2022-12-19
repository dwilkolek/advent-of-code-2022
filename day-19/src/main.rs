// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Blueprint {
    id: i32,
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost: (i32, i32),
    geode_robot_cost: (i32, i32),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    time: i32,
    ore: i32,
    ore_robots: i32,
    clay: i32,
    clay_robots: i32,
    obsidian: i32,
    obsidian_robots: i32,
    geode: i32,
    geode_robots: i32,
}

impl State {
    fn new() -> Self {
        Self {
            time: 0,
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geode: 0,
            geode_robots: 0,
        }
    }

    fn collect(&mut self) {
        self.time += 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }
}

fn main() {
    let regex = Regex::new("Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();
    let mut blueprints: Vec<Blueprint> = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for line in input.into_iter() {
            let cap = regex.captures(&line).unwrap();
            blueprints.push(Blueprint {
                id: cap.get(1).unwrap().as_str().parse().unwrap(),
                ore_robot_cost: cap.get(2).unwrap().as_str().parse().unwrap(),
                clay_robot_cost: cap.get(3).unwrap().as_str().parse().unwrap(),
                obsidian_robot_cost: (
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                    cap.get(5).unwrap().as_str().parse().unwrap(),
                ),
                geode_robot_cost: (
                    cap.get(6).unwrap().as_str().parse().unwrap(),
                    cap.get(7).unwrap().as_str().parse().unwrap(),
                ),
            });
        }
    }
    let mut part1 = 0;
    for blueprint in blueprints.iter() {
        println!("Blueprint {} ", blueprint.id);
        part1 += test(blueprint, 24) * blueprint.id;
    }

    println!("Part 1: {}", part1);

    let mut part2 = 1;
    for blueprint in blueprints[0..3].to_vec().iter() {
        println!("Blueprint {} ", blueprint.id);
        part2 *= test(blueprint, 32);
    }

    println!("Part 2: {}", part2);
}

fn test(blueprint: &Blueprint, time_limit: i32) -> i32 {
    let mut uniqe_states = HashSet::new();
    let mut deq: VecDeque<State> = VecDeque::new();
    let mut best = 0;

    deq.push_back(State::new());

    while let Some(mut state) = deq.pop_front() {
        if state.geode > best {
            best = state.geode
        }
        if state.geode < best - 1 || uniqe_states.contains(&state) {
            continue;
        };
        uniqe_states.insert(state);

        if state.time == time_limit {
            continue;
        }

        if state.ore >= blueprint.geode_robot_cost.0
            && state.obsidian >= blueprint.geode_robot_cost.1
        {
            let mut next = State {
                ore: state.ore - blueprint.geode_robot_cost.0,
                obsidian: state.obsidian - blueprint.geode_robot_cost.1,
                ..state
            };
            next.collect();
            next.geode_robots += 1;
            deq.push_back(next);
        }
        if blueprint.obsidian_robot_cost.0 <= state.ore
            && blueprint.obsidian_robot_cost.1 <= state.clay
        {
            let mut next = State {
                ore: state.ore - blueprint.obsidian_robot_cost.0,
                clay: state.clay - blueprint.obsidian_robot_cost.1,
                ..state
            };
            next.collect();
            next.obsidian_robots += 1;
            deq.push_back(next);
        }
        if blueprint.ore_robot_cost <= state.ore {
            let mut next = State {
                ore: state.ore - blueprint.ore_robot_cost,
                ..state
            };
            next.collect();
            next.ore_robots += 1;
            deq.push_back(next);
        }
        if blueprint.clay_robot_cost <= state.ore {
            let mut next = State {
                ore: state.ore - blueprint.clay_robot_cost,
                ..state
            };
            next.collect();
            next.clay_robots += 1;
            deq.push_back(next);
        }
        state.collect();
        deq.push_back(state);
    }

    best
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

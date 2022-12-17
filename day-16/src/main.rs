// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;
struct Planner {
    valves: HashMap<String, usize>,
    routes: HashMap<String, Vec<String>>,
    shortest_paths: HashMap<(String, String), usize>,
}

impl Planner {
    fn new(valves: HashMap<String, usize>, routes: HashMap<String, Vec<String>>) -> Planner {
        let shortest_paths = Planner::all_shortest_routes(&valves, &routes);
        Planner {
            valves,
            routes,
            shortest_paths,
        }
    }

    fn reasonable_valves(&self, traveler: &TravelState) -> Vec<(String, usize)> {
        let mut opts: Vec<(String, usize)> = vec![];
        for path in self.shortest_paths.clone() {
            let position_matching = path.0 .0 == traveler.position;
            let not_opened = !traveler.opened_valves.contains_key(&path.0 .1);
            let has_budget = traveler.minutes_left > path.1;

            if position_matching & not_opened & has_budget {
                opts.push((path.0 .1, path.1 + 1))
            }
        }
        // let opts: Vec<(String, usize)> = self
        //     .shortest_paths
        //     .clone()
        //     .into_iter()
        //     .filter(|path| {
        //         path.0 .1 == traveler.position
        //             && !traveler.opened_valves.contains_key(&path.0 .1)
        //             && traveler.minutes_left > path.1
        //     })
        //     .map(|path| (path.0 .1, path.1))
        //     .collect();
        // println!("{:?}", opts);
        return opts;
    }

    fn all_shortest_routes(
        valves: &HashMap<String, usize>,
        routes: &HashMap<String, Vec<String>>,
    ) -> HashMap<(String, String), usize> {
        let mut all: HashMap<(String, String), usize> = HashMap::new();
        let mut deq = VecDeque::new();
        let meaningfull_valves: Vec<String> = valves
            .clone()
            .into_iter()
            .filter(|v| v.1 > 0 || v.0 == "AA")
            .map(|v| v.0)
            .collect();

        println!("Meaningful valves {:?}", meaningfull_valves);
        for from in meaningfull_valves.clone().into_iter() {
            for to in meaningfull_valves.clone().into_iter() {
                if from != to {
                    all.insert((from.clone(), to.to_owned()), usize::MAX);
                    deq.push_back(vec![from.clone()]);
                }
            }
        }

        while let Some(trace) = deq.pop_front() {
            let current_position = trace.last().unwrap().clone();
            let start = trace.first().unwrap().clone();
            if let Some(dist) = all.get(&(start.clone(), current_position.clone())) {
                if dist > &(trace.len() - 1) {
                    all.insert((start.clone(), current_position.clone()), trace.len() - 1);
                }
            }
            let available_routes = routes.get(&current_position).unwrap();
            for route in available_routes.into_iter() {
                if !trace.contains(route) {
                    let mut next_trace = trace.clone();
                    next_trace.push(route.clone());
                    deq.push_back(next_trace);
                }
            }
        }

        println!("Index: {:?}", all);
        all
    }
}

fn main() {
    let mut valves: HashMap<String, usize> = HashMap::new();
    let mut routes: HashMap<String, Vec<String>> = HashMap::new();

    let reg = Regex::new(
        "Valve ([A-Z]+) has flow rate=([0-9]+); tunnel[s]{0,1} lead[s]{0,1} to valve[s]{0,1} (.*)",
    )
    .unwrap();

    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();

        for line in input.into_iter() {
            println!("Line {}", line);
            let cap = reg.captures(&line).unwrap();
            let valve_id = cap.get(1).unwrap().as_str().to_owned();
            valves.insert(
                valve_id.clone(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
            );

            routes.insert(
                valve_id.clone(),
                cap.get(3)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(String::from)
                    .collect(),
            );
        }
    }
    let planner = Planner::new(valves, routes);

    let best = next_action(
        &planner,
        TravelState {
            position: "AA".to_owned(),
            minutes_left: 30,
            opened_valves: HashMap::new(),
            log: vec![],
        },
    );

    println!("{:?}. {:?}", best, best.flow_rate(&planner.valves))
    // let mut ope = HashMap::new();
    // ope.insert("EE".to_owned(), 9);
    // ope.insert("HH".to_owned(), 13);
    // ope.insert("BB".to_owned(), 25);
    // ope.insert("JJ".to_owned(), 21);
    // ope.insert("DD".to_owned(), 28);
    // ope.insert("CC".to_owned(), 4);
    // let best = TravelState {
    //     flow_rate: 1,
    //     history: vec![],
    //     opened_valves: ope.clone(),
    //     minutes_left: 0,
    //     position: "AA".to_owned(),
    //     visited_valves: vec![],
    // };
    // let mut c_flow = 0;
    // for min in 0..30 {
    //     for o in ope.clone().into_iter() {
    //         if o.1 == (30 - min) {
    //             c_flow += valves.get(o.0.clone()).un
    //         }
    //     }
    //     println!("Minut {}, flow: {}", min, c_flow)
    // }
}

#[derive(Clone, Debug)]
struct TravelState {
    position: String,
    minutes_left: usize,
    opened_valves: HashMap<String, usize>,
    log: Vec<String>,
}

impl TravelState {
    fn action(&self, next_valve: String, cost: usize) -> TravelState {
        let mut new_valves = self.opened_valves.clone();
        let minutes_after_move = self.minutes_left - cost;
        new_valves.insert(next_valve.clone(), minutes_after_move);

        let mut c_log = self.log.clone();

        c_log.push(format!(
            "Action to: {}. Cost: {}. Minutes left: {}.",
            next_valve, cost, minutes_after_move
        ));

        TravelState {
            position: next_valve,
            minutes_left: minutes_after_move,
            opened_valves: new_valves,
            log: c_log,
        }
    }

    fn flow_rate(&self, valves: &HashMap<String, usize>) -> usize {
        let mut flow_rate = 0;
        let mut sum = 0;
        for (valve_id, at_minute) in self.opened_valves.iter() {
            flow_rate += valves.get(valve_id).unwrap() * at_minute;
        }
        flow_rate
    }
}

fn next_action(planner: &Planner, state: TravelState) -> TravelState {
    let options = planner.reasonable_valves(&state);
    let mut best_state = state.clone();
    for option in options.into_iter() {
        let next_state = next_action(planner, state.action(option.0, option.1));
        if next_state.flow_rate(&planner.valves) > best_state.flow_rate(&planner.valves) {
            best_state = next_state;
        }
    }

    best_state
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Clone, Debug)]
struct Planner {
    valves: HashMap<String, usize>,
    shortest_paths: HashMap<(String, String), usize>,
}

struct TravelPlan {
    next_valve: String,
    cost: usize,
}

impl Planner {
    fn new(valves: HashMap<String, usize>, routes: HashMap<String, Vec<String>>) -> Planner {
        let shortest_paths = Planner::all_shortest_routes(&valves, &routes);
        Planner {
            valves,
            shortest_paths,
        }
    }

    fn reasonable_valves(&self, state: &TravelState) -> Vec<TravelPlan> {
        let traveler = &state.traveler;
        let mut opts: Vec<TravelPlan> = vec![];
        for path in self.shortest_paths.clone() {
            let position_matching = path.0 .0 == traveler.position;
            let not_opened = !state.opened_valves.contains_key(&path.0 .1);
            let has_budget = traveler.minutes_left > path.1;

            if position_matching & not_opened & has_budget {
                opts.push(TravelPlan {
                    next_valve: path.0 .1,
                    cost: path.1 + 1,
                });
            }
        }
        if opts.len() > 0 {
            opts.sort_by(|a, b| {
                self.valves
                    .get(&b.next_valve)
                    .unwrap()
                    .cmp(self.valves.get(&a.next_valve).unwrap())
            });
            return opts;
        }
        return vec![];
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
                if !trace.contains(route) && trace.len() < 30 {
                    let mut next_trace = trace.clone();
                    next_trace.push(route.clone());
                    deq.push_back(next_trace);
                }
            }
        }
        all
    }
}

#[derive(Clone, Debug)]
struct TravelState {
    traveler: Traveler,
    opened_valves: HashMap<String, usize>,
}

#[derive(Clone, Debug)]
struct Traveler {
    id: String,
    position: String,
    minutes_left: usize,
    minutes_total: usize,
}

impl TravelState {
    fn route(&self) -> Vec<String> {
        return self.opened_valves.clone().into_keys().collect();
    }
    fn action(&self, plan: TravelPlan) -> TravelState {
        let mut new_opened_valves = self.opened_valves.clone();

        let minutes_after_move = self.traveler.minutes_left - plan.cost;
        let new_traveler = Traveler {
            id: self.traveler.id.clone(),
            position: plan.next_valve.clone(),
            minutes_left: minutes_after_move,
            minutes_total: self.traveler.minutes_total,
        };
        new_opened_valves.insert(plan.next_valve.clone(), minutes_after_move);

        TravelState {
            traveler: new_traveler,
            opened_valves: new_opened_valves,
        }
    }

    fn flow_rate(&self, valves: &HashMap<String, usize>) -> usize {
        let mut flow_rate = 0;
        for (valve_id, at_minute) in self.opened_valves.iter() {
            flow_rate += valves.get(valve_id).unwrap() * at_minute;
        }
        flow_rate
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

    println!("Start processing");

    let planner = Planner::new(valves, routes);
    let mut routes_cache = vec![];
    let part_1 = next_action(
        &planner,
        TravelState {
            traveler: Traveler {
                id: "me".to_owned(),
                position: "AA".to_owned(),
                minutes_left: 30,
                minutes_total: 30,
            },
            opened_valves: HashMap::new(),
        },
        false,
        &mut routes_cache,
    );

    println!("Part 1: {:?}", part_1.flow_rate(&planner.valves));

    let mut routes_cache = vec![];
    let part_2 = next_action(
        &planner,
        TravelState {
            traveler: Traveler {
                id: "me".to_owned(),
                position: "AA".to_owned(),
                minutes_left: 26,
                minutes_total: 26,
            },
            opened_valves: HashMap::new(),
        },
        true,
        &mut routes_cache,
    );

    let thr = part_2.flow_rate(&planner.valves);
    println!("Part 2 max single: {:?}", thr);
    let mut max = 0;
    let mut routes = routes_cache;

    routes.sort_by(|a, b| b.0.cmp(&a.0));

    let rlen = routes.len();
    let mut possible_best = usize::MAX;
    for route_1_i in 0..rlen {
        for route_2_i in route_1_i + 1..rlen {
            if route_1_i == 0 && route_2_i == route_1_i + 1 {
                possible_best = routes[route_2_i].0 + routes[route_1_i].0;
                println!("Possible best: {}", possible_best);
            }
            let now = routes[route_2_i].0 + routes[route_1_i].0;
            if now < thr || now < max || possible_best < max {
                break;
            }
            let mut unique = true;
            let route_1 = &routes[route_1_i];
            let route_2 = &routes[route_2_i];
            for v in route_2.1.clone() {
                if route_1.1.contains(&v) {
                    unique = false;
                    break;
                }
            }
            if unique && max < route_1.0 + route_2.0 {
                max = route_1.0 + route_2.0;
                println!("Current part 2 {}", max);
                if max >= possible_best {
                    return;
                }
            }
        }
    }
    println!("Part 2 {}", max);
}

fn next_action(
    planner: &Planner,
    state: TravelState,
    skip_collecting_routes: bool,
    routes: &mut Vec<(usize, Vec<String>)>,
) -> TravelState {
    let plans = planner.reasonable_valves(&state);
    let mut best_state = state.clone();
    if skip_collecting_routes {
        let cache_entry = (best_state.flow_rate(&planner.valves), state.route());
        if !routes.contains(&cache_entry) {
            routes.push(cache_entry)
        }
    }
    for plan in plans.into_iter() {
        let next_state = next_action(planner, state.action(plan), skip_collecting_routes, routes);
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

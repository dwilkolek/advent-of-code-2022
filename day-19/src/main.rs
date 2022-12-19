// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc;
use std::{thread, vec};

use regex::Regex;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Mat {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl Display for Mat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mat::Clay => write!(f, "Clay"),
            Mat::Obsidian => write!(f, "Obsidian"),
            Mat::Ore => write!(f, "Ore"),
            Mat::Geode => write!(f, "Geode"),
        }
    }
}
const MATERIAL_ORDER: [Mat; 4] = [Mat::Geode, Mat::Obsidian, Mat::Clay, Mat::Ore];

#[derive(Debug, Clone)]
struct Robot {
    cost: Vec<(Mat, usize)>,
    material: Mat,
    active: bool,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    stash: HashMap<Mat, usize>,
    robot_blueprints: HashMap<Mat, Robot>,
    robots: Vec<Robot>,
    time_left: usize,
    log: Vec<String>,
    first_g: usize,
    first_o: usize,
    first_c: usize,
}

impl Blueprint {
    fn quality(&self) -> usize {
        match self.stash.get(&Mat::Geode) {
            Some(count) => count * self.id,
            None => 0 * self.id,
        }
    }

    fn robots(&self, m: Mat) -> usize {
        self.robots.iter().filter(|r| r.material == m).count()
    }

    fn materials_stashed() {

        // self.stash
        //     .clone()
        //     .into_iter()
        //     .map(|s| match s {
        //         (Mat::Ore, c) => c,
        //         (Mat::Clay, c) => 1000 * c,
        //         (Mat::Obsidian, c) => 1000 * c,
        //         (Mat::Geode, c) => 10000 * c,
        //     })
        //     .sum(),
    }

    fn unique_id(
        &self,
    ) -> (
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) {
        (
            self.id,
            self.time_left,
            *self.stash.get(&Mat::Ore).unwrap_or(&0),
            *self.stash.get(&Mat::Clay).unwrap_or(&0),
            *self.stash.get(&Mat::Obsidian).unwrap_or(&0),
            *self.stash.get(&Mat::Geode).unwrap_or(&0),
            self.robots(Mat::Ore),
            self.robots(Mat::Clay),
            self.robots(Mat::Obsidian),
            self.robots(Mat::Geode),
        )
    }
}

fn create_robots(blueprint: &Blueprint) -> Vec<Blueprint> {
    let mut alternative_realities = vec![];
    alternative_realities.push(blueprint.clone()); //noop

    for mat in MATERIAL_ORDER {
        let rb = blueprint.robot_blueprints.get(&mat).unwrap();
        let mut can_produce = true;
        for mat_cost in rb.cost.iter() {
            can_produce &= blueprint.stash.get(&mat_cost.0).unwrap() >= &mat_cost.1;
        }
        if can_produce {
            let mut producing = blueprint.clone();
            producing.robots.push(rb.clone());
            for mat_cost in rb.cost.iter() {
                match producing.stash.get(&mat_cost.0) {
                    Some(count) => {
                        producing
                            .stash
                            .insert(mat_cost.0.clone(), count - mat_cost.1);
                    }
                    None => {
                        panic!("Not enough materials")
                    }
                }
            }
            producing.log.push(format!(
                "Creating {} robot. Timeleft: {}",
                &mat, &producing.time_left
            ));
            alternative_realities.push(producing);
        }
    }
    alternative_realities.sort_by(|a, b| b.robots(Mat::Geode).cmp(&a.robots(Mat::Geode)));
    alternative_realities
}

fn collect_mats(blueprint: &mut Blueprint) {
    for r in blueprint.robots.iter() {
        if r.active {
            if r.material == Mat::Geode {
                blueprint.first_g = blueprint.first_g.min(blueprint.time_left)
            }
            blueprint.log.push(format!("Collected {}.", &r.material));
            match blueprint.stash.get(&r.material) {
                Some(count) => {
                    blueprint.stash.insert(r.material.clone(), count + 1);
                }
                None => {
                    blueprint.stash.insert(r.material.clone(), 1);
                }
            }
        }
    }
}
fn activate_robots(blueprint: &mut Blueprint) {
    blueprint.time_left -= 1;

    blueprint.log.push(format!("Stash {:?}", blueprint.stash));
    blueprint
        .log
        .push(format!("== Minute {} ==", 25 - blueprint.time_left));

    for r in blueprint.robots.iter_mut() {
        r.active = true;
    }
}

fn find_best(
    mut blueprint: Blueprint,
    cache: &mut HashSet<(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    )>,
    first_g: &mut HashMap<usize, usize>,
    first_o: &mut HashMap<usize, usize>,
    first_c: &mut HashMap<usize, usize>,
) -> Option<Blueprint> {
    cache.insert(blueprint.unique_id());

    if blueprint.first_g >= *first_g.get(&blueprint.id).unwrap_or(&0) {
        first_g.insert(blueprint.id, blueprint.first_g);
    } else {
        return None;
    }
    // if blueprint.first_o >= *first_o.get(&blueprint.id).unwrap_or(&0) {
    //     first_o.insert(blueprint.id, blueprint.first_o);
    // } else {
    //     return None;
    // }
    // if blueprint.first_c >= *first_c.get(&blueprint.id).unwrap_or(&0) {
    //     first_c.insert(blueprint.id, blueprint.first_c);
    // } else {
    //     return None;
    // }

    collect_mats(&mut blueprint);
    activate_robots(&mut blueprint);
    if blueprint.time_left != 0 {
        let paths = create_robots(&blueprint);
        let mut best = blueprint;
        for path in paths {
            if !cache.contains(&path.unique_id()) {
                let opportunity = find_best(path, cache, first_g, first_o, first_c);
                if let Some(opportunity) = opportunity {
                    if opportunity.quality() > best.quality() {
                        best = opportunity;
                    }
                }
            }
        }
        Some(best)
    } else {
        Some(blueprint)
    }
}

fn main() {
    let regex = Regex::new("Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();

    let mut blueprints: Vec<Blueprint> = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for line in input.into_iter() {
            let cap = regex.captures(&line).unwrap();
            let mut robot_blueprints = HashMap::new();
            robot_blueprints.insert(
                Mat::Ore,
                Robot {
                    active: false,
                    material: Mat::Ore,
                    cost: vec![(Mat::Ore, cap.get(2).unwrap().as_str().parse().unwrap())],
                },
            );
            robot_blueprints.insert(
                Mat::Clay,
                Robot {
                    active: false,
                    material: Mat::Clay,
                    cost: vec![(Mat::Ore, cap.get(3).unwrap().as_str().parse().unwrap())],
                },
            );
            robot_blueprints.insert(
                Mat::Obsidian,
                Robot {
                    active: false,
                    material: Mat::Obsidian,
                    cost: vec![
                        (Mat::Ore, cap.get(4).unwrap().as_str().parse().unwrap()),
                        (Mat::Clay, cap.get(5).unwrap().as_str().parse().unwrap()),
                    ],
                },
            );
            robot_blueprints.insert(
                Mat::Geode,
                Robot {
                    active: false,
                    material: Mat::Geode,
                    cost: vec![
                        (Mat::Ore, cap.get(6).unwrap().as_str().parse().unwrap()),
                        (Mat::Obsidian, cap.get(7).unwrap().as_str().parse().unwrap()),
                    ],
                },
            );

            let mut stash = HashMap::new();
            stash.insert(Mat::Clay, 0);
            stash.insert(Mat::Geode, 0);
            stash.insert(Mat::Obsidian, 0);
            stash.insert(Mat::Ore, 0);
            blueprints.push(Blueprint {
                time_left: 25,
                id: cap.get(1).unwrap().as_str().parse().unwrap(),
                stash,
                robot_blueprints,
                log: vec![],
                first_g: 0,
                first_o: 0,
                first_c: 0,
                robots: vec![Robot {
                    active: false,
                    cost: vec![],
                    material: Mat::Ore,
                }],
            });
        }
    }

    let (tx, rx) = mpsc::channel();

    let mut handlers = vec![];
    for blueprint in blueprints {
        let tx = tx.clone();
        let h = thread::spawn(move || {
            println!("Started Blueprint {}", blueprint.id);
            let mut cache = HashSet::new();
            let mut first_g = HashMap::new();
            let mut first_o = HashMap::new();
            let mut first_c = HashMap::new();
            let quality = find_best(
                blueprint.clone(),
                &mut cache,
                &mut first_g,
                &mut first_o,
                &mut first_c,
            )
            .unwrap()
            .quality();
            // result += quality;
            println!("Blueprint {} done, quality {}", blueprint.id, quality);
            tx.send(quality).unwrap();
        });
        handlers.push(h)
    }

    for h in handlers {
        h.join().unwrap()
    }
    let mut result = 0;
    for received in rx {
        println!("Got: {}", received);
        result += received;
    }

    println!("Results: {:?}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

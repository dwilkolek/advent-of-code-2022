// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Clone)]
struct Ap {
    sensor: (i32, i32),
    beacon: (i32, i32),
    dist: i32,
}

impl Ap {
    fn get_coverage(&self, y: i32) -> Vec<(i32, i32)> {
        let mut cov: Vec<_> = vec![];
        let dist = Ap::dist(self.sensor, self.beacon);
        for x in self.sensor.0 - dist..=self.sensor.0 + dist {
            if Ap::dist(self.sensor, (x, y)) <= dist {
                cov.push((x, y))
            }
        }

        cov
    }

    fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
        return (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i32;
    }

    fn range(&self, y: i32) -> Option<(i32, i32)> {
        if y > self.sensor.1 + self.dist {
            return None;
        }
        if y < self.sensor.1 - self.dist {
            return None;
        }
        let aa: i32 = self.sensor.1.abs_diff(y) as i32;
        let diff_at_y = self.dist - aa;
        let result = (
            self.sensor.0 - diff_at_y.abs(),
            self.sensor.0 + diff_at_y.abs(),
        );
        return Some(result);
    }

    fn new(sensor: (i32, i32), beacon: (i32, i32)) -> Ap {
        Ap {
            sensor,
            beacon,
            dist: Ap::dist(sensor, beacon),
        }
    }
}

fn main() {
    let reg = Regex::new("Sensor at x=([-]{0,1}[0-9]+), y=([-]{0,1}[0-9]+): closest beacon is at x=([-]{0,1}[0-9]+), y=([-]{0,1}[0-9]+)").unwrap();

    let mut aps: Vec<Ap> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();

        for line in input.into_iter() {
            let cap = reg.captures(&line).unwrap();

            aps.push(Ap::new(
                (
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                ),
            ));
        }
    }
    part1(&aps);
    part2(&aps);
}

fn part1(aps: &Vec<Ap>) {
    let y = 2000000;
    let mut coverage_on_y: HashSet<(i32, i32)> = HashSet::new();
    for ap in aps.into_iter() {
        ap.get_coverage(y).into_iter().for_each(|p| {
            if p.1 == y {
                coverage_on_y.insert(p);
            }
        })
    }
    println!("Coverage on Y({}) is: {}", y, coverage_on_y.len() - 1);
}

fn part2(aps: &Vec<Ap>) {
    let max = 4000000;
    for y in 0..4000000 {
        let mut used_ranges: Vec<_> = Vec::new();
        for ap in aps {
            let range = ap.range(y);
            if let Some(range) = range {
                used_ranges.push((range.0.max(0), range.1.min(max)))
            }
        }
        used_ranges.sort();

        let mut current = 0;
        loop {
            match find_next_end(current, &used_ranges) {
                Some(next) => current = next,
                None => {
                    let c: i128 = current as i128;
                    println!("Part 2 tuning freq: {}", (c + 1) * 4000000 + y as i128);
                    return;
                }
            }
            if current >= max {
                break;
            }
        }
    }
}

fn find_next_end(current: i32, used_ranges: &Vec<(i32, i32)>) -> Option<i32> {
    let best: Vec<i32> = used_ranges
        .into_iter()
        .filter(|ur| ur.0 <= current && ur.1 > current)
        .map(|ur| ur.1)
        .collect();
    best.last().cloned()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

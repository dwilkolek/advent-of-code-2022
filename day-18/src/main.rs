// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
    exposed: usize,
    neighbours: Vec<(usize, usize, usize)>,
}

impl Cube {
    fn new(x: usize, y: usize, z: usize) -> Cube {
        Cube {
            x,
            y,
            z,
            exposed: 6,
            neighbours: vec![(x, y, z)],
        }
    }
}
fn main() {
    let mut cubes = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for line in input {
            let coords: Vec<usize> = line
                .split(",")
                .map(|coor| coor.to_string().parse().unwrap())
                .collect();
            cubes.push(Cube::new(coords[0], coords[1], coords[2]))
        }
    }
    let mut cubes = cubes.clone();

    for i in 0..cubes.len() {
        for j in i..cubes.len() {
            touching(&mut cubes, i, j);
        }
    }
    let total_surface: usize = cubes.iter().map(|c| c.exposed).sum();
    println!("Part 1: {:?}", total_surface);

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut max_z: usize = 0;

    let mut min_x: usize = usize::MAX;
    let mut min_y: usize = usize::MAX;
    let mut min_z: usize = usize::MAX;

    let mut space: Vec<Vec<Vec<usize>>> = vec![];

    for cube in cubes.iter() {
        max_x = max_x.max(cube.x);
        max_y = max_y.max(cube.y);
        max_z = max_z.max(cube.z);
        min_x = min_x.min(cube.x);
        min_y = min_y.min(cube.y);
        min_z = min_z.min(cube.z);
    }
    max_x = max_x - min_x + 2;
    max_y = max_y - min_y + 2;
    max_z = max_z - min_z + 2;

    for x in 0..=max_x {
        space.push(vec![]);
        for y in 0..=max_y {
            space[x].push(vec![]);
            for _ in 0..=max_z {
                space[x][y].push(0);
            }
        }
    }
    for c in cubes {
        space[c.x - min_x + 1][c.y - min_y + 1][c.z - min_z + 1] = 1;
    }

    let mut visited: HashSet<_> = HashSet::new();
    let mut deq: VecDeque<(usize, usize, usize)> = VecDeque::new();
    deq.push_back((0, 0, 0));
    space[0][0][0] = 2;
    visited.insert((0, 0, 0));
    let dirs: &Vec<(isize, isize, isize)> = &vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];
    while let Some(p) = deq.pop_front() {
        for dir in dirs {
            let px = p.0 as isize + dir.0;
            let py = p.1 as isize + dir.1;
            let pz = p.2 as isize + dir.2;
            if px < 0 || py < 0 || pz < 0 {
                continue;
            }
            let sur_point = &(px as usize, py as usize, pz as usize);
            if let Some(x) = space.get_mut(sur_point.0) {
                if let Some(y) = x.get_mut(sur_point.1) {
                    if let Some(z) = y.get_mut(sur_point.2) {
                        if !visited.contains(&sur_point.clone()) {
                            visited.insert(sur_point.clone());
                            if *z == 0 {
                                space[sur_point.0][sur_point.1][sur_point.2] = 2;
                                deq.push_back(sur_point.clone())
                            }
                        }
                    }
                }
            }
        }
    }

    let mut air_pockets: Vec<Cube> = vec![];
    for x in 0..space.len() {
        for y in 0..space[x].len() {
            for z in 0..space[x][y].len() {
                if space[x][y][z] == 0 {
                    air_pockets.push(Cube::new(x, y, z))
                }
            }
        }
    }

    for i in 0..air_pockets.len() {
        for j in i..air_pockets.len() {
            touching(&mut air_pockets, i, j);
        }
    }
    let pocket_surface = air_pockets.iter().map(|c| c.exposed).sum::<usize>();

    println!("Part 2: {}", total_surface - pocket_surface);
    println!("  Pockets suraface: {}", pocket_surface);
}

fn touching(cubes: &mut Vec<Cube>, i: usize, j: usize) {
    let diff = cubes[i].x.abs_diff(cubes[j].x)
        + cubes[i].y.abs_diff(cubes[j].y)
        + cubes[i].z.abs_diff(cubes[j].z);
    if diff == 1 {
        cubes[i].exposed -= 1;
        cubes[j].exposed -= 1;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

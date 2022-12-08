// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        let mut trees: Vec<Vec<usize>> = vec![];
        for input_line in input.into_iter() {
            let row_of_trees: Vec<usize> = input_line
                .chars()
                .map(|char| (char.to_string()).parse::<usize>().unwrap())
                .collect();
            trees.push(row_of_trees)
        }

        // let rows = trees.len();
        // let cols = trees[0].len();
        // println!("Tree map [r: {}, c: {}] = {:?}", rows, cols, trees);

        let mut inner_visible = 0;
        for (row_i, row) in trees.clone().into_iter().enumerate() {
            for (col_i, tree) in row.into_iter().enumerate() {
                if is_visible(&trees, tree, row_i, col_i) {
                    inner_visible = inner_visible + 1;
                }
            }
        }
        println!("visible: {}", inner_visible);

        let mut max_dist = 0;
        for (row_i, row) in trees.clone().into_iter().enumerate() {
            for (col_i, tree) in row.into_iter().enumerate() {
                let view_dist = greatest_view_distance(&trees, tree, row_i, col_i);

                if view_dist > max_dist {
                    max_dist = view_dist;
                }
            }
        }
        println!("Max view dist: {}", max_dist);
    }
}

fn greatest_view_distance(
    trees: &Vec<Vec<usize>>,
    tree: usize,
    tree_row_i: usize,
    tree_col_i: usize,
) -> usize {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    let mut top: Vec<usize> = vec![];
    let mut bottom: Vec<usize> = vec![];
    for (row_i, row) in trees.into_iter().enumerate() {
        for (col_i, value) in row.into_iter().enumerate() {
            if row_i == tree_row_i && col_i < tree_col_i {
                let mut tmp = left.clone();
                let mut new_left = vec![value.clone()];
                new_left.append(&mut tmp);
                left = new_left;
            }
            if row_i == tree_row_i && col_i > tree_col_i {
                right.push(value.clone())
            }
            if row_i < tree_row_i && col_i == tree_col_i {
                let mut tmp = top.clone();
                let mut new_top = vec![value.clone()];
                new_top.append(&mut tmp);
                top = new_top;
            }
            if row_i > tree_row_i && col_i == tree_col_i {
                bottom.push(value.clone())
            }
        }
    }

    let l = view_distance(tree, left);
    let r = view_distance(tree, right);
    let t = view_distance(tree, top);
    let b = view_distance(tree, bottom);
    let res = l * r * t * b;
    // println!(
    //     "Tree({})[{},{}] = l:{} r:{} t:{} b:{} --> {}",
    //     tree, tree_row_i, tree_col_i, l, r, t, b, res
    // );
    return res;
}

fn view_distance(tree: usize, view: Vec<usize>) -> usize {
    let mut seen = 0;
    for t in view.into_iter() {
        seen = seen + 1;
        if t >= tree {
            break;
        }
    }
    seen
}

fn is_visible(trees: &Vec<Vec<usize>>, tree: usize, tree_row_i: usize, tree_col_i: usize) -> bool {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    let mut top: Vec<usize> = vec![];
    let mut bottom: Vec<usize> = vec![];
    for (row_i, row) in trees.into_iter().enumerate() {
        for (col_i, value) in row.into_iter().enumerate() {
            if row_i == tree_row_i && col_i < tree_col_i {
                left.push(value.clone())
            }
            if row_i == tree_row_i && col_i > tree_col_i {
                right.push(value.clone())
            }
            if row_i < tree_row_i && col_i == tree_col_i {
                top.push(value.clone())
            }
            if row_i > tree_row_i && col_i == tree_col_i {
                bottom.push(value.clone())
            }
        }
    }

    let l = left.into_iter().filter(|t| t >= &tree).count();
    let r = right.into_iter().filter(|t| t >= &tree).count();
    let t = top.into_iter().filter(|t| t >= &tree).count();
    let b = bottom.into_iter().filter(|t| t >= &tree).count();
    let res = l == 0 || r == 0 || t == 0 || b == 0;
    // println!(
    //     "Tree({})[{},{}] = l:{} r:{} t:{} b:{} --> {}",
    //     tree, tree_row_i, tree_col_i, l, r, t, b, res
    // );
    return res;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

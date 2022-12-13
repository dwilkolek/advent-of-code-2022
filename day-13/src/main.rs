// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Element {
    Single(i64),
    List(Vec<Element>),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x = match (self, other) {
            (Element::Single(a), Element::Single(b)) => Some(a.cmp(b)),
            (Element::Single(a), Element::List(_)) => {
                Element::List(vec![Element::Single(*a)]).partial_cmp(other)
            }
            (Element::List(_), Element::Single(b)) => {
                self.partial_cmp(&Element::List(vec![Element::Single(*b)]))
            }
            (Element::List(a), Element::List(b)) => {
                for (a, b) in a.iter().zip(b) {
                    let cmp = a.partial_cmp(&b);
                    if let Some(Ordering::Equal) = cmp {
                        continue;
                    }
                    return cmp;
                }
                Some(a.len().cmp(&b.len()))
            }
        };
        // println!("Compare {:?} with {:?} = {:?}", self, other, x);
        x
    }
}

pub fn from_value(v: &Value) -> Element {
    if v.is_array() {
        Element::List(
            v.as_array()
                .unwrap()
                .into_iter()
                .map(|iv| from_value(iv))
                .collect(),
        )
    } else {
        Element::Single(v.as_i64().unwrap())
    }
}

fn main() {
    let mut pairs: Vec<(Element, Element)> = vec![];
    let mut elements: Vec<Element> = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for chunk in input.chunks(3).into_iter() {
            let a: Value = serde_json::from_str(chunk[0].as_str()).unwrap();
            let b: Value = serde_json::from_str(chunk[1].as_str()).unwrap();
            let a = from_value(&a);
            let b = from_value(&b);
            pairs.push((a.clone(), b.clone()));
            elements.push(a);
            elements.push(b);
        }
    }

    let mut part_1 = 0;
    for (index, pair) in pairs.into_iter().enumerate() {
        let result = compute(pair.0, pair.1);
        // println!("== Pair {} == -> {}", index + 1, result);
        if result {
            part_1 += index + 1
        }
    }
    println!("Part 1 result: {}", part_1);

    let dividers = vec![from_value(&json!([[2]])), from_value(&json!([[6]]))];
    elements.append(&mut dividers.clone());
    elements.sort_by(|e1, e2| e1.partial_cmp(e2).unwrap());
    let mut decoder_key = 1;
    for (index, element) in elements.into_iter().enumerate() {
        if dividers.contains(&element) {
            decoder_key *= index + 1
        }
    }
    println!("Part 2 result: {}", decoder_key);
}

pub fn compute(a: Element, b: Element) -> bool {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) => true,
        _ => false,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn test_compute(a: &Value, b: &Value) -> bool {
        compute(from_value(a), from_value(b))
    }

    #[test]
    fn example_1() {
        assert_eq!(
            test_compute(&json!([1, 1, 3, 1, 1]), &json!([1, 1, 5, 1, 1])),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            test_compute(&json!([[1], [2, 3, 4]]), &json!([[1], 4])),
            true
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(test_compute(&json!([9]), &json!([[8, 7, 6]])), false);
    }

    #[test]
    fn example_4() {
        assert_eq!(
            test_compute(&json!([[4, 4], 4, 4]), &json!([[4, 4], 4, 4, 4])),
            true
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(test_compute(&json!([7, 7, 7, 7]), &json!([7, 7, 7])), false);
    }

    #[test]
    fn example_6() {
        assert_eq!(test_compute(&json!([]), &json!([3])), true);
    }

    #[test]
    fn example_7() {
        assert_eq!(test_compute(&json!([[[]]]), &json!([[]])), false);
    }

    #[test]
    fn example_8() {
        assert_eq!(
            test_compute(
                &json!([1, [2, [3, [4, [5, 6, 7]]]], 8, 9]),
                &json!([1, [2, [3, [4, [5, 6, 0]]]], 8, 9])
            ),
            false
        );
    }
}

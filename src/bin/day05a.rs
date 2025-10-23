use std::collections::{HashMap, HashSet};

fn init_constraints() -> HashMap<String, HashSet<String>> {
    let mut constraints: HashMap<String, HashSet<String>> = HashMap::new();
    include_str!("../../inputs/day05a.txt")
        .lines()
        .for_each(|x| {
            let (l, r) = x.split_once('|').unwrap();
            constraints
                .entry(l.to_string())
                .or_default()
                .insert(r.to_string());
        });
    constraints
}

fn is_valid(line: &[&str], constraints: &HashMap<String, HashSet<String>>) -> bool {
    for (a, bs) in constraints {
        for b in bs {
            if let (Some(i), Some(j)) = (
                line.iter().position(|&x| x == a),
                line.iter().position(|&x| x == b),
            ) && i > j
            {
                return false;
            }
        }
    }
    true
}

fn get_mid(line: &[&str]) -> i32 {
    line[line.len() / 2].parse::<i32>().unwrap()
}

fn main() {
    let content: Vec<Vec<&str>> = include_str!("../../inputs/day05b.txt")
        .lines()
        .map(|x| x.split(',').collect())
        .collect();
    let constraints = init_constraints();
    let result: i32 = content
        .iter()
        .filter_map(|x| {
            if is_valid(x, &constraints) {
                Some(get_mid(x))
            } else {
                None
            }
        })
        .sum();
    println!("{:?}", result);
}

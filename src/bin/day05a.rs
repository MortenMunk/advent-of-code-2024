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

fn kahn(line: Vec<&str>) -> i32 {}

fn main() {
    let content: Vec<Vec<&str>> = include_str!("../../inputs/day05b.txt")
        .lines()
        .map(|x| x.split(',').collect())
        .collect();
    let constraints = init_constraints();
    println!("{:?}", content)
}

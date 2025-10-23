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

fn kahn(line: &Vec<&str>, constraints: &HashMap<String, HashSet<String>>) -> i32 {
    let mut l: Vec<String> = Vec::new();
    let mut s: HashSet<String> = line
        .iter()
        .filter(|&&x| constraints.contains_key(x))
        .map(|&x| x.to_string())
        .collect();

    let mut indegree: HashMap<String, usize> = HashMap::new();
    for n in constraints.keys() {
        indegree.entry(n.clone()).or_insert(0);
        if let Some(neighbors) = constraints.get(n) {
            for m in neighbors {
                *indegree.entry(m.clone()).or_insert(0) += 1;
            }
        }
    }

    while let Some(n) = s.iter().next().cloned() {
        s.remove(&n);
        l.push(n.clone());

        if let Some(neighbors) = constraints.get(&n) {
            for m in neighbors {
                if let Some(deg) = indegree.get_mut(m)
                    && *deg > 0
                {
                    *deg -= 1;
                    if *deg == 0 {
                        s.insert(m.clone());
                    }
                }
            }
        }
    }

    l[l.len() / 2].parse::<i32>().unwrap()
}

fn main() {
    let content: Vec<Vec<&str>> = include_str!("../../inputs/day05b.txt")
        .lines()
        .map(|x| x.split(',').collect())
        .collect();
    let constraints = init_constraints();
    let result: i32 = content.iter().map(|x| kahn(x, &constraints)).sum();
    println!("{:?}", result)
}

use std::collections::HashMap;

fn main() {
    let (left, right): (Vec<i32>, Vec<i32>) = include_str!("../../inputs/day01.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    let freqs: HashMap<i32, i32> = right.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    let similarity_score: i32 = left
        .iter()
        .map(|&x| {
            let count = freqs.get(&x).copied().unwrap_or(0);
            x * count
        })
        .sum();

    println!("{:?}", similarity_score);
}

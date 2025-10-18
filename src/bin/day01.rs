fn main() {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = include_str!("../../inputs/day01.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();
    left.sort();
    right.sort();

    let distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("{:?}", distance);
}

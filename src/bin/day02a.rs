fn main() {
    fn split_and_parse(line: &str) -> Vec<i32> {
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }

    let lines: Vec<Vec<i32>> = include_str!("../../inputs/day02.txt")
        .lines()
        .map(split_and_parse)
        .collect();

    let safe_count: usize = lines
        .iter()
        .filter(|x| {
            let increase = x
                .windows(2)
                .all(|pair| pair[1] > pair[0] && pair[1] - pair[0] <= 3);

            let decrease = x
                .windows(2)
                .all(|pair| pair[1] < pair[0] && pair[0] - pair[1] <= 3);
            increase || decrease
        })
        .count();

    println!("{:?}", safe_count);
}

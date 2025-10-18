fn main() {
    fn split_and_parse(line: &str) -> Vec<i32> {
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }

    fn is_safe(v: &[i32]) -> bool {
        let increase = v.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])));
        let decrease = v.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1])));
        increase || decrease
    }

    fn is_safe_with_one_removal(v: &[i32]) -> bool {
        if v.len() < 2 {
            return true;
        };

        for i in 0..v.len() {
            let mut copy = v.to_vec();
            copy.remove(i);
            if is_safe(&copy) {
                return true;
            }
        }
        false
    }

    let lines: Vec<Vec<i32>> = include_str!("../../inputs/day02.txt")
        .lines()
        .map(split_and_parse)
        .collect();

    let safe_count: usize = lines.iter().filter(|x| is_safe_with_one_removal(x)).count();

    println!("{:?}", safe_count);
}

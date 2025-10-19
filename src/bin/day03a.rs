fn calculate_mul(buffer: &[char]) -> i32 {
    let s: String = buffer.iter().collect();
    if let Some(start) = s.find('(') {
        let inside = &s[start + 1..s.len()];
        let parts: Vec<&str> = inside.split(',').collect();
        let a: i32 = parts[0].parse().unwrap();
        let b: i32 = parts[1].parse().unwrap();
        a * b
    } else {
        0
    }
}

fn main() {
    let mut buffer: Vec<char> = Vec::new();
    let mut has_comma: bool = false;
    let mut acc: i32 = 0;
    let stream: Vec<&str> = include_str!("../../inputs/day03.txt").lines().collect();

    for line in stream.iter() {
        for token in line.chars() {
            if token == 'm' && buffer.is_empty() {
                buffer.push(token);
            } else if token == 'u' && buffer.len() == 1 {
                buffer.push(token);
            } else if token == 'l' && buffer.len() == 2 {
                buffer.push(token);
            } else if token == '(' && buffer.len() == 3 {
                buffer.push(token);
            } else if token.is_ascii_digit() && buffer.len() >= 4 {
                buffer.push(token);
            } else if token == ',' && buffer.len() >= 5 && !has_comma {
                has_comma = true;
                buffer.push(token);
            } else if token == ')' && buffer.len() >= 7 && has_comma {
                acc += calculate_mul(&buffer);
                has_comma = false;
                buffer.clear();
            } else {
                has_comma = false;
                buffer.clear();
            }
        }
    }
    println!("{:?}", acc);
}

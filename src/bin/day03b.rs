fn calculate_mul(buf: &[char]) -> i32 {
    let s: String = buf.iter().collect();
    if let (Some(l), Some(r)) = (s.find('('), s.find(')')) {
        let inside = &s[l + 1..r];
        let parts: Vec<&str> = inside.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                return a * b;
            }
        }
    }
    0
}

fn main() {
    let mut acc: i32 = 0;
    let mut enabled = true;
    let mut buffer: Vec<char> = Vec::new();
    let mut has_comma = false;

    let input = include_str!("../../inputs/day03.txt");
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 3 < chars.len() && &chars[i..i + 4] == ['d', 'o', '(', ')'] {
            enabled = true;
            i += 4;
            continue;
        }
        if i + 6 < chars.len() && &chars[i..i + 7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {
            enabled = false;
            i += 7;
            continue;
        }

        if enabled {
            let ch = chars[i];
            if ch == 'm' && buffer.is_empty() {
                buffer.push(ch);
            } else if ch == 'u' && buffer.len() == 1 {
                buffer.push(ch);
            } else if ch == 'l' && buffer.len() == 2 {
                buffer.push(ch);
            } else if ch == '(' && buffer.len() == 3 {
                buffer.push(ch);
            } else if ch.is_ascii_digit() && buffer.len() >= 4 {
                buffer.push(ch);
            } else if ch == ',' && buffer.len() >= 5 && !has_comma {
                has_comma = true;
                buffer.push(ch);
            } else if ch.is_ascii_digit() && has_comma {
                buffer.push(ch);
            } else if ch == ')' && has_comma {
                buffer.push(ch);
                acc += calculate_mul(&buffer);
                buffer.clear();
                has_comma = false;
            } else {
                if ch == 'm' {
                    buffer.clear();
                    buffer.push('m');
                    has_comma = false;
                } else if !ch.is_whitespace() {
                    buffer.clear();
                    has_comma = false;
                }
            }
        }

        i += 1;
    }

    println!("{acc}");
}

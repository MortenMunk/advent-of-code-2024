fn columns(matrix: &[Vec<char>]) -> i32 {
    let cols = matrix.first().map(|r| r.len()).unwrap_or(0);
    (0..cols)
        .map(|c| {
            let col: Vec<_> = matrix
                .iter()
                .filter_map(|row| row.get(c).copied())
                .collect();
            col.windows(4)
                .filter(|x| x == &['X', 'M', 'A', 'S'] || x == &['S', 'A', 'M', 'X'])
                .count() as i32
        })
        .sum()
}

fn rows(matrix: &[Vec<char>]) -> i32 {
    matrix
        .iter()
        .map(|r| {
            r.windows(4)
                .filter(|x| x == &['X', 'M', 'A', 'S'] || x == &['S', 'A', 'M', 'X'])
                .count() as i32
        })
        .sum()
}

fn r_diagonals(matrix: &[Vec<char>]) -> i32 {
    if matrix.is_empty() {
        return 0;
    }

    let rows = matrix.len();
    let cols = matrix.iter().map(|r| r.len()).max().unwrap_or(0);
    (0..cols)
        .map(|c| (0, c))
        .chain((1..rows).map(|r| (r, 0)))
        .map(|(r0, c0)| {
            let diagonal: Vec<_> = (0..)
                .map_while(|i| matrix.get(r0 + i).and_then(|row| row.get(c0 + i)).copied())
                .collect();
            diagonal
                .windows(4)
                .filter(|x| x == &['X', 'M', 'A', 'S'] || x == &['S', 'A', 'M', 'X'])
                .count() as i32
        })
        .sum()
}

fn l_diagonals(matrix: &[Vec<char>]) -> i32 {
    let rows = matrix.len();
    let cols = matrix.iter().map(|r| r.len()).max().unwrap_or(0);
    (0..cols)
        .map(|c| (0, c))
        .chain((1..rows).map(|r| (r, cols - 1)))
        .map(|(r0, c0)| {
            let diagonal: Vec<_> = (0..)
                .map_while(|i| {
                    let col = c0.checked_sub(i)?;
                    matrix.get(r0 + i).and_then(|row| row.get(col)).copied()
                })
                .collect();
            diagonal
                .windows(4)
                .filter(|x| x == &['X', 'M', 'A', 'S'] || x == &['S', 'A', 'M', 'X'])
                .count() as i32
        })
        .sum()
}

fn main() {
    let matrix: Vec<Vec<char>> = include_str!("../../inputs/day04.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let count = rows(&matrix) + columns(&matrix) + r_diagonals(&matrix) + l_diagonals(&matrix);
    println!("{:?}", count);
}

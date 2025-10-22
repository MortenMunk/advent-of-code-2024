fn x_mas(matrix: &[Vec<char>]) -> i32 {
    if matrix.is_empty() {
        return 0;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut count: i32 = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if matrix[r][c] == 'A' {
                let nw = matrix[r - 1][c - 1];
                let ne = matrix[r - 1][c + 1];
                let sw = matrix[r + 1][c - 1];
                let se = matrix[r + 1][c + 1];

                if ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M'))
                    && ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M'))
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let matrix: Vec<Vec<char>> = include_str!("../../inputs/day04.txt")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let count = x_mas(&matrix);
    println!("{:?}", count)
}

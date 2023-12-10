pub fn run() -> i32 {
    let matrix: Vec<Vec<char>> = super::utils::INPUT
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows: usize = matrix.len() as usize;
    let cols: usize = matrix[0].len() as usize;

    let mut sum = 0;

    let mut i = 0;
    let mut j = 0;

    while i < rows {
        while j < cols {
            if matrix[i][j].is_ascii_digit() {
                let start_column = j;
                let mut number: String = String::new();

                while matrix[i][j].is_ascii_digit() {
                    let digit = matrix[i][j];
                    number.push(digit);
                    j += 1;
                    if j >= cols {
                        break;
                    }
                }

                let end_column = j - 1;

                if is_adjacent_to_symbol(start_column, end_column, i, &matrix) {
                    sum += number.parse::<i32>().unwrap();
                } else {
                }
            }

            j += 1;
            if j >= cols {
                break;
            }
        }

        i += 1;
        j = 0;
    }

    return sum;
}

fn is_symbol(char: char) -> bool {
    char != '.' && !char.is_ascii_digit()
}

fn is_adjacent_to_symbol(x1: usize, x2: usize, y: usize, data: &Vec<Vec<char>>) -> bool {
    let adj_y1 = std::cmp::max(y, 1) - 1;
    let adj_y2 = std::cmp::min(y + 1, data.len() - 1);

    let adj_x1 = std::cmp::max(x1, 1) - 1;
    let adj_x2 = std::cmp::min(x2 + 1, data[0].len() - 1);

    for row in adj_y1..=adj_y2 {
        for x in adj_x1..=adj_x2 {
            if row == y && (x >= x1 && x <= x2) {
                continue;
            }
            if is_symbol(data[row][x]) {
                return true;
            }
        }
    }

    false
}

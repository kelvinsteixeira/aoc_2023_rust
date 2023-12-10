pub fn run() -> i32 {
    
    let matrix: Vec<Vec<char>> = super::utils::INPUT
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows: usize = matrix.len() as usize;
    let cols: usize = matrix[0].len() as usize;

    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            if is_gear(matrix[i][j]) {
                let gear_ratio: i32 = get_gear_ratio(i, j, &matrix);
                if gear_ratio > 0 {
                    sum += gear_ratio;
                }
            }
        }
    }

    return sum;
}

fn is_gear(char: char) -> bool {
    char == '*'
}

fn get_gear_ratio(row: usize, col: usize, data: &Vec<Vec<char>>) -> i32 {
    let top_line_values = search_nested_values(row - 1, col, data);
    let bottom_line_values = search_nested_values(row + 1, col, data);
    let middle_line_values = search_nested_values(row, col, data);

    let all_values = top_line_values
        .iter()
        .chain(bottom_line_values.iter())
        .chain(middle_line_values.iter())
        .collect::<Vec<&i32>>();

    if all_values.len() == 2 {
        return all_values[0] * all_values[1];
    }

    return 0;
}

fn search_nested_values(row: usize, col: usize, data: &Vec<Vec<char>>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![];
    let mut col = col;

    if data[row][col].is_ascii_digit() {
        while data[row][col].is_ascii_digit() {
            col -= 1;
        }

        col += 1;
        let mut number = String::new();
        while data[row][col].is_ascii_digit() {
            number.push(data[row][col]);
            col += 1;
        }

        if number.len() > 0 {
            results.push(number.parse::<i32>().unwrap());
        }
    } else {
        let number_right = check_right(row, col, data);
        let number_left = check_left(row, col, data);

        if number_right > 0 {
            results.push(number_right);
        }

        if number_left > 0 {
            results.push(number_left);
        }
    }

    return results;
}

fn check_right(row: usize, col: usize, data: &Vec<Vec<char>>) -> i32 {
    let mut col = col + 1;

    if col < data[row].len() && data[row][col].is_ascii_digit() {
        let mut number = String::new();
        while col < data[row].len() && data[row][col].is_ascii_digit() {
            number.push(data[row][col]);
            col += 1;
        }

        return number.parse::<i32>().unwrap();
    }

    return 0;
}

fn check_left(row: usize, col: usize, data: &Vec<Vec<char>>) -> i32 {
    let mut col = col - 1;

    if col > 0 && data[row][col].is_ascii_digit() {
        let mut number = String::new();

        while data[row][col].is_ascii_digit() {
            number.push(data[row][col]);

            if col == 0 {
                break;
            }

            col -= 1;
        }

        number = number.chars().rev().collect::<String>();
        return number.parse::<i32>().unwrap();
    }

    return 0;
}

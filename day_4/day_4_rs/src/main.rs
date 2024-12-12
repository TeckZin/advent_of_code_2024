// use std::fs;
//
// fn main() {
//     let content = match read_file() {
//         Ok(content) => content,
//         Err(err) => {
//             println!("err {}", err);
//             String::new()
//         }
//     };
//     part_1(content);
// }
//
// fn read_file() -> Result<String, std::io::Error> {
//     return fs::read_to_string("src/sample.txt");
// }
//
//
// fn part_1(content: String) {
//     let mut total = 0;
//     println!("{}", content);
//
//     fn get_char_at_pos(content_row: &Vec<&str>, row: usize, col: usize) -> Option<char> {
//         content_row.get(row)?.chars().nth(col)
//     }
//
//     let content_rows: Vec<&str> = content.lines().collect();
//     // dia
//
//     let rows = content_rows.len();
//     let cols = content_rows[0].chars().count();
//
//     let mut dias: Vec<String> = Vec::new();
//     for start_col in 0..cols {
//         let mut dia = String::new();
//         let mut row = 0;
//         let mut col = start_col;
//
//         while row < rows && col < cols {
//             if let Some(ch) = get_char_at_pos(&content_rows, row, col) {
//                 dia.push(ch);
//             }
//             row += 1;
//             col += 1;
//         }
//         if dia.len() > 3 {
//             dias.push(dia);
//         }
//     }
//     // Additional main diagonals starting from first column
//     for start_row in 1..rows {
//         let mut dia = String::new();
//         let mut row = start_row;
//         let mut col = 0;
//
//         while row < rows && col < cols {
//             if let Some(ch) = get_char_at_pos(&content_rows, row, col) {
//                 dia.push(ch);
//             }
//             row += 1;
//             col += 1;
//         }
//         if dia.len() > 3 {
//             dias.push(dia);
//         }
//     }
//
//     // Anti-diagonals (top-right to bottom-left)
//     for start_col in (0..cols).rev() {
//         let mut dia = String::new();
//         let mut row = 0;
//         let mut col = start_col;
//
//         while row < rows && col > 0 {
//             if let Some(ch) = get_char_at_pos(&content_rows, row, col) {
//                 dia.push(ch);
//             }
//             row += 1;
//             col -= 1;
//         }
//         if dia.len() > 3 {
//             dias.push(dia);
//         }
//     }
//
//     // Additional anti-diagonals starting from last column
//     for start_row in 1..rows {
//         let mut dia = String::new();
//         let mut row = start_row;
//         let mut col = cols - 1;
//
//         while row < rows && col > 0 {
//             if let Some(ch) = get_char_at_pos(&content_rows, row, col) {
//                 dia.push(ch);
//             }
//             row += 1;
//             col -= 1;
//         }
//         if dia.len() > 3 {
//             dias.push(dia);
//         }
//     }
//
//     println!("dia: {:#?}", dias);
//
//     for d in &dias {
//         println!("d {}", d);
//         for (index, _) in d.match_indices("X") {
//             if index + 4 < d.len() {
//                 let slice = &d[index..index + 4];
//                 if slice == "XMAS" {
//                     total += 1;
//                     // println!("{}", slice);
//                     // println!("next\n");
//                 }
//             }
//         }
//         let rev_d: String = d.chars().rev().collect();
//
//         for (index, _) in rev_d.match_indices("X") {
//             if index + 4 < d.len() {
//                 let slice = &rev_d[index..index + 4];
//                 if slice == "XMAS" {
//                     total += 1;
//                 }
//             }
//         }
//     }
//
//     // also works
//     // println!("{:?}", content_rows);
//     println!("{:#?}", content_rows);
//     let mut content_columns: Vec<String> = vec![String::new(); content_rows.len()];
//     for line in content.lines() {
//         for (i, c) in line.chars().enumerate() {
//             content_columns[i].push(c);
//         }
//     }
//
//     for col in &content_columns {
//         println!("col {}", col);
//         for (index, _) in col.match_indices("X") {
//             if index + 4 < col.len() {
//                 let slice = &col[index..index + 4];
//                 if slice == "XMAS" {
//                     total += 1;
//                     // println!("{}", slice);
//                     // println!("next\n");
//                 }
//             }
//         }
//
//         let rev_col: String = col.chars().rev().collect();
//
//         for (index, _) in rev_col.match_indices("X") {
//             if index + 4 < col.len() {
//                 let slice = &rev_col[index..index + 4];
//                 if slice == "XMAS" {
//                     total += 1;
//                 }
//             }
//         }
//     }
//
//     // println!("{:#?}", content_columns);
//     for row in &content_rows {
//         // println!("row {}", row);
//
//         // let index: i32 = match row.find('X') {
//         //     Some(index) => index as i32,
//         //     None => -1,
//         // };
//         // if index >= 0 {
//         //     if let Some(char) = row.chars().nth(index as usize) {
//         //         println!("here {}", char);
//         //     }
//         // }
//         //
//         //
//         // if let Some(index) = row.find('X') {
//         //     // let char = row.chars().nth(index).unwrap();
//         //     // println!("{}", char);
//         //     let slice = &row[index..index + 4];
//         //     println!("{}", slice);
//         // }
//         //
//         //
//
//         // HORIZONTAL
//
//         for (index, _) in row.match_indices("X") {
//             if index + 4 < row.len() {
//                 let slice = &row[index..index + 4];
//                 if slice == "XMAS" {
//                     total += 1;
//                     // println!("{}", slice);
//                     // println!("next\n");
//                 }
//             }
//         }
//
//         let rev_row: String = row.chars().rev().collect();
//
//         for (index, _) in rev_row.match_indices("X") {
//             if index + 4 < row.len() {
//                 let slice = &rev_row[index..index + 4];
//                 if slice == "XMAS" {
//                     total += 1;
//                     // println!("{}", slice);
//                     // println!("next\n");
//                 }
//             }
//         }
//     }
//     println!("total: {}", total);
// }
//
use std::io;

fn main() -> io::Result<()> {
    // Read input from file
    let input = std::fs::read_to_string("src/sample.txt")?;

    // Parse input into grid
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    if !grid.is_empty() {
        println!("Part 1: {}", part_1(&grid));
        println!("Part 2: {}", part_2(&grid));
    }

    Ok(())
}

fn part_1(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    let target = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            // Check all 8 directions
            if check_right(grid, row, col, &target, cols) {
                count += 1;
            }
            if check_down(grid, row, col, &target, rows) {
                count += 1;
            }
            if check_diagonal_right_down(grid, row, col, &target, rows, cols) {
                count += 1;
            }
            if check_diagonal_right_up(grid, row, col, &target, cols) {
                count += 1;
            }
            if check_left(grid, row, col, &target) {
                count += 1;
            }
            if check_up(grid, row, col, &target) {
                count += 1;
            }
            if check_diagonal_left_up(grid, row, col, &target) {
                count += 1;
            }
            if check_diagonal_left_down(grid, row, col, &target, rows) {
                count += 1;
            }
        }
    }
    count
}

fn part_2(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    if rows < 3 {
        // Need at least 3 rows for an X pattern
        return 0;
    }
    let cols = grid[0].len();
    if cols < 3 {
        // Need at least 3 columns for an X pattern
        return 0;
    }

    let mut count = 0;

    // For each possible center point of the X
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            // Check if this position could be the center of an X-MAS
            if grid[row][col] == 'A' {
                count += check_x_pattern(grid, row, col);
            }
        }
    }

    count
}

// Part 1 helper functions
fn check_right(grid: &[Vec<char>], row: usize, col: usize, target: &[char], cols: usize) -> bool {
    if col + 3 >= cols {
        return false;
    }
    for i in 0..4 {
        if grid[row][col + i] != target[i] {
            return false;
        }
    }
    true
}

fn check_down(grid: &[Vec<char>], row: usize, col: usize, target: &[char], rows: usize) -> bool {
    if row + 3 >= rows {
        return false;
    }
    for i in 0..4 {
        if grid[row + i][col] != target[i] {
            return false;
        }
    }
    true
}

fn check_diagonal_right_down(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    target: &[char],
    rows: usize,
    cols: usize,
) -> bool {
    if row + 3 >= rows || col + 3 >= cols {
        return false;
    }
    for i in 0..4 {
        if grid[row + i][col + i] != target[i] {
            return false;
        }
    }
    true
}

fn check_diagonal_right_up(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    target: &[char],
    cols: usize,
) -> bool {
    if row < 3 || col + 3 >= cols {
        return false;
    }
    for i in 0..4 {
        if grid[row - i][col + i] != target[i] {
            return false;
        }
    }
    true
}

fn check_left(grid: &[Vec<char>], row: usize, col: usize, target: &[char]) -> bool {
    if col < 3 {
        return false;
    }
    for i in 0..4 {
        if grid[row][col - i] != target[i] {
            return false;
        }
    }
    true
}

fn check_up(grid: &[Vec<char>], row: usize, col: usize, target: &[char]) -> bool {
    if row < 3 {
        return false;
    }
    for i in 0..4 {
        if grid[row - i][col] != target[i] {
            return false;
        }
    }
    true
}

fn check_diagonal_left_up(grid: &[Vec<char>], row: usize, col: usize, target: &[char]) -> bool {
    if row < 3 || col < 3 {
        return false;
    }
    for i in 0..4 {
        if grid[row - i][col - i] != target[i] {
            return false;
        }
    }
    true
}

fn check_diagonal_left_down(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    target: &[char],
    rows: usize,
) -> bool {
    if row + 3 >= rows || col < 3 {
        return false;
    }
    for i in 0..4 {
        if grid[row + i][col - i] != target[i] {
            return false;
        }
    }
    true
}

// Part 2 helper functions
fn check_x_pattern(grid: &[Vec<char>], center_row: usize, center_col: usize) -> usize {
    let mut valid_patterns = 0;

    // Check all combinations of MAS directions
    valid_patterns += check_diagonal_pair(grid, center_row, center_col, true, true)
        + check_diagonal_pair(grid, center_row, center_col, true, false)
        + check_diagonal_pair(grid, center_row, center_col, false, true)
        + check_diagonal_pair(grid, center_row, center_col, false, false);

    valid_patterns
}

fn check_diagonal_pair(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    first_forward: bool,
    second_forward: bool,
) -> usize {
    // Check if both diagonals form valid MAS sequences
    if check_diagonal(grid, row, col, -1, -1, first_forward) &&    // Upper-left
       check_diagonal(grid, row, col, -1, 1, second_forward)
    // Upper-right
    {
        1
    } else {
        0
    }
}

fn check_diagonal(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    dx: i32,
    dy: i32,
    forward: bool,
) -> bool {
    let sequence = if forward {
        ['M', 'A', 'S']
    } else {
        ['S', 'A', 'M']
    };

    // We're already at 'A' (center), need to check 'M' and 'S'
    // Check the position before A
    let prev_row = (row as i32 + dx) as usize;
    let prev_col = (col as i32 + dy) as usize;

    // Check the position after A
    let next_row = (row as i32 - dx) as usize;
    let next_col = (col as i32 - dy) as usize;

    // Boundary checks
    if prev_row >= grid.len()
        || prev_col >= grid[0].len()
        || next_row >= grid.len()
        || next_col >= grid[0].len()
    {
        return false;
    }

    // Check if the characters match the expected sequence
    grid[prev_row][prev_col] == sequence[0]
        && grid[row][col] == sequence[1]
        && grid[next_row][next_col] == sequence[2]
}

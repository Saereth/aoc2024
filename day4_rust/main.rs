use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn count_word_occurrences(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    //I mean... it's not pretty but it works
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (1, -1),  // diagonal down-left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (-1, 1),  // diagonal up-right
    ];

    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    //aaaand here we go
    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                if (0..word_len).all(|k| {
                    let new_row = row as isize + dr * k as isize;
                    let new_col = col as isize + dc * k as isize;
                    new_row >= 0
                        && new_row < rows as isize
                        && new_col >= 0
                        && new_col < cols as isize
                        && grid[new_row as usize][new_col as usize] == word_chars[k]
                }) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_xmas_pattern(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Look for the xmas pattern
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            //Messy again but it works
            let top_left = grid[row - 1][col - 1];
            let top_right = grid[row - 1][col + 1];
            let center = grid[row][col];
            let bottom_left = grid[row + 1][col - 1];
            let bottom_right = grid[row + 1][col + 1];

            // Match the "X-MAS" pattern
            if center == 'A' &&
                ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M')) &&
                ((top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M')) {
                count += 1;
            }
        }
    }

    count
}

fn read_grid_from_file(file_path: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                grid.push(row.chars().collect());
            }
        }
    }
    grid
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let file_path = "d4.csv";
    let grid = read_grid_from_file(file_path);

    // Part 1 output
    let word_to_find = "XMAS";
    let total_occurrences = count_word_occurrences(&grid, word_to_find);
    println!("Part 1: Total occurrences of '{}': {}", word_to_find, total_occurrences);

    // Part 2 output
    let xmas_pattern_occurrences = count_xmas_pattern(&grid);
    println!("Part 2: Total occurrences of 'X-MAS' pattern: {}", xmas_pattern_occurrences);
}

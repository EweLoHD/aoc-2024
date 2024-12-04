use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let puzzle: Vec<Vec<char>> = reader.lines().map(|l| {
            l.unwrap().chars().collect()
        }).collect();

        let mut answer = 0;

        for (y, l) in puzzle.iter().enumerate() {
            for (x, &c) in l.iter().enumerate() {
                if c == 'X' {
                    // Check right
                    if puzzle[y].len() > x+3 && puzzle[y][x+1] == 'M' && puzzle[y][x+2] == 'A' && puzzle[y][x+3] == 'S' {
                        answer += 1;
                    }

                    // Check left
                    if x >= 3 && puzzle[y][x-1] == 'M' && puzzle[y][x-2] == 'A' && puzzle[y][x-3] == 'S' {
                        answer += 1;
                    }

                    // Check down
                    if puzzle.len() > y+3 && puzzle[y+1][x] == 'M' && puzzle[y+2][x] == 'A' && puzzle[y+3][x] == 'S' {
                        answer += 1;
                    }

                    // Check up
                    if y >= 3 && puzzle[y-1][x] == 'M' && puzzle[y-2][x] == 'A' && puzzle[y-3][x] == 'S' {
                        answer += 1;
                    }

                    // Check right down
                    if puzzle[y].len() > x+3 && puzzle.len() > y+3 && puzzle[y+1][x+1] == 'M' && puzzle[y+2][x+2] == 'A' && puzzle[y+3][x+3] == 'S' {
                        answer += 1;
                    }

                    // Check left up
                    if x >= 3 && y >= 3 && puzzle[y-1][x-1] == 'M' && puzzle[y-2][x-2] == 'A' && puzzle[y-3][x-3] == 'S' {
                        answer += 1;
                    }

                    // Check right up
                    if puzzle[y].len() > x+3 && y >= 3 && puzzle[y-1][x+1] == 'M' && puzzle[y-2][x+2] == 'A' && puzzle[y-3][x+3] == 'S' {
                        answer += 1;
                    }

                    // Check left down
                    if x >= 3 && puzzle.len() > y+3 && puzzle[y+1][x-1] == 'M' && puzzle[y+2][x-2] == 'A' && puzzle[y+3][x-3] == 'S' {
                        answer += 1;
                    }

                }
            }

        }

        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let puzzle: Vec<Vec<char>> = reader.lines().map(|l| {
            l.unwrap().chars().collect()
        }).collect();

        let mut answer = 0;

        for (y, l) in puzzle.iter().enumerate() {
            for (x, &c) in l.iter().enumerate() {
                if c == 'A' {
                    if puzzle[y].len() > x+1
                        && puzzle.len() > y+1
                        && x >= 1
                        && y >= 1
                    {
                        if ((puzzle[y-1][x-1] == 'M' && puzzle[y+1][x+1] == 'S') || (puzzle[y-1][x-1] == 'S' && puzzle[y+1][x+1] == 'M'))
                            && ((puzzle[y+1][x-1] == 'M' && puzzle[y-1][x+1] == 'S') || (puzzle[y+1][x-1] == 'S' && puzzle[y-1][x+1] == 'M'))
                        {
                            answer += 1;
                        }
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

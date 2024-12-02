use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let reports: Vec<Vec<i32>> = reader.lines().map(|line| {
            line.unwrap().split_whitespace().map(|l| l.parse::<i32>().unwrap()).collect_vec()
        }).collect();

        let answer = reports.iter().filter(|r| is_safe(*r)).count();

        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion


    fn is_safe(r: &Vec<i32>) -> bool {
        let mut min: i32 = r[0] - r[1];
        let mut max: i32 = r[0] - r[1];

        let mut prev = r[1];

        r.iter().skip(2).for_each(|l| {
            let d = prev - *l;

            if d > max {
                max = d;
            } else if d < min {
                min = d;
            }

            prev = *l;
        });

        if max.abs() <= 3 && min.abs() <= 3 && ((max > 0 && min > 0) || (max < 0 && min < 0)) {
            true
        } else {
            false
        }
    }


    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let reports: Vec<Vec<i32>> = reader.lines().map(|line| {
            line.unwrap().split_whitespace().map(|l| l.parse::<i32>().unwrap()).collect_vec()
        }).collect();

        let answer = reports.iter().filter(|&r| {
            if is_safe(r) {
                true
            } else {
                for i in 0..r.len() {
                    let mut rm = r.clone();
                    rm.remove(i);

                    if is_safe(&rm) {
                        return true
                    }
                }
                false
            }
        }).count();

        Ok(answer as usize)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

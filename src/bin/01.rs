use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::write;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::izip;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        //let answer = reader.lines().flatten().count();

        let (mut left, mut right): (Vec<i32>, Vec<i32>) = reader.lines()
            .map(|line| line.unwrap())
            .map(|line| {
                let mut iter = line.split_whitespace();

                (iter.next().unwrap().parse::<i32>().unwrap(), iter.next().unwrap().parse::<i32>().unwrap())
            })
            .unzip();

        left.sort();
        right.sort();

        let answer: i32 = izip!(left, right).map(|(l, r)| {
            (l - r).abs()
        }).sum();

        Ok(answer as usize)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (mut left, mut right): (Vec<i32>, Vec<i32>) = reader.lines()
            .map(|line| line.unwrap())
            .map(|line| {
                let mut iter = line.split_whitespace();

                (iter.next().unwrap().parse::<i32>().unwrap(), iter.next().unwrap().parse::<i32>().unwrap())
            })
            .unzip();

        left.sort();
        right.sort();

        let answer = left.iter().map(|l| {
            l * right.iter().filter(|r| l == *r).count() as i32
        }).sum::<i32>();

        Ok(answer as usize)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

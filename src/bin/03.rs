use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).expect("error");

        let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)")?;

        let answer: i32 = re.captures_iter(&*input).map(|c| {
            c.get(1).unwrap().as_str()
                .split_terminator(',')
                .map(|s| s.parse::<i32>().unwrap())
                .product::<i32>()
        }).sum();

        //println!("{:?}", answer);

        Ok(answer as usize)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).expect("error");

        let re = Regex::new(r"(mul\(([0-9]+,[0-9]+)\))|(do\(\))|(don't\(\))")?;

        let mut enabled = true;

        let answer: i32 = re.captures_iter(&*input).map(|c| {
            //println!("{:?}", c);

            let str = c.get(0).unwrap().as_str();

            if str.starts_with("mul") && enabled {
                return c.get(2).unwrap().as_str()
                    .split_terminator(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .product::<i32>()
            } else if str.eq("do()") {
                enabled = true;
            } else {
                enabled = false;
            }

            0
        }).sum();

        //println!("{:?}", answer);

        Ok(answer as usize)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

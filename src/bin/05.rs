use std::arch::x86_64::_mm256_broadcastd_epi32;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

/*struct Page {
    num: i32,
    smaller: Vec<i32>,
    larger: Vec<i32>
}

impl Page {
    pub fn new(num: i32) -> Page {
        Page {
            num,
            smaller: Vec::new(),
            larger: Vec::new(),
        }
    }

    pub fn add_smaller (&self, )
}*/

fn main() -> Result<()> {
    start_day(DAY);

    fn parse_input<R: BufRead>(reader: &mut R, ordering_rules: &mut Vec<(i32, i32)>, updates: &mut Vec<Vec<i32>>) {
        let mut second_part = false;
        reader.lines().for_each(|l| {
            let l = l.unwrap();

            if !second_part {
                if l.is_empty() {
                    second_part = true;
                } else {
                    ordering_rules.push(l.split_once("|").
                        map_or((0, 0), |(x, y)| {
                            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                        })
                    )
                }
            } else {
                updates.push(l.split_terminator(',').map(|x| x.parse::<i32>().unwrap()).collect());
            }
        });
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut ordering_rules: Vec<(i32, i32)> = Vec::new();
        let mut updates: Vec<Vec<i32>> = Vec::new();

        parse_input(&mut reader, &mut ordering_rules, &mut updates);

        //println!("{:?}", ordering_rules);
        //println!("{:?}", updates);

        let answer: i32 = updates.iter().filter(|&u| {
            for x in 0..u.len() {
                for y in (x+1)..u.len() {
                    if ordering_rules.contains(&(u[y], u[x])) {
                        return false
                    }
                }
            }

            true
        }).map(|x| {
            x[x.len()/2]
        }).sum();

        Ok(answer as usize)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut ordering_rules: Vec<(i32, i32)> = Vec::new();
        let mut updates: Vec<Vec<i32>> = Vec::new();

        parse_input(&mut reader, &mut ordering_rules, &mut updates);

        let answer: i32 = updates.iter_mut().filter_map(|u| {
            let mut filter = false;

            for x in 0..u.len() {
                for y in (x+1)..u.len() {
                    if ordering_rules.contains(&(u[y], u[x])) {
                        filter = true;

                        u.swap(x, y);
                    }
                }
            }

            if filter {Some(u)} else {None}
        }).map(|x| {
            x[x.len()/2]
        }).sum();

        Ok(answer as usize)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

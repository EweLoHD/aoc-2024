use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "2333133121414131402";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        //let mut input = String::new();
        //reader.read_to_string(&mut input).expect("Could no read input");

        let mut input = reader.lines().next().unwrap()?;

        let mut disk: Vec<Option<i32>> = Vec::new();
        let mut is_space = false;

        for (i, c) in input.chars().enumerate() {
            if !is_space {
                disk.append(&mut vec![Some(i as i32 / 2); c.to_digit(10).unwrap() as usize]);
                is_space = true;
            } else {
                disk.append(&mut vec![None; c.to_digit(10).unwrap() as usize]);
                is_space = false;
            }
        }

        /*println!("{}", disk.iter().map(|x| {
            if x.is_some() {
                x.unwrap().to_string()
            } else {
                ".".parse().unwrap()
            }
        }).collect::<String>());*/

        let mut done = false;

        //for (i, &d) in disk.iter().enumerate() {
        for i in 0..disk.len() {
            if done {
                break;
            }

            if disk[i].is_none() {
                for j in (0..disk.len()).rev() {
                    if j <= i {
                        done = true;
                        break;
                    }

                    if disk[j].is_some() {
                        disk.swap(i, j);
                        break;
                    }
                }
            }
        }

        /*println!("{}", disk.iter().map(|x| {
            if x.is_some() {
                x.unwrap().to_string()
            } else {
                ".".parse().unwrap()
            }
        }).collect::<String>());*/
        
        let answer: i64 = disk.iter().enumerate().map(|(i, &id)| {
            /*if !id.is_none() {
                (i as i32) * id.unwrap_
            } else {
                0
            }*/
            (i as i64) * id.unwrap_or(0) as i64
        }).sum();

        Ok(answer as usize)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    //let result = time_snippet!(part1(input_file)?);
    //println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut input = reader.lines().next().unwrap()?;

        let mut disk: Vec<Option<i32>> = Vec::new();
        let mut is_space = false;

        for (i, c) in input.chars().enumerate() {
            if !is_space {
                disk.append(&mut vec![Some(i as i32 / 2); c.to_digit(10).unwrap() as usize]);
                is_space = true;
            } else {
                disk.append(&mut vec![None; c.to_digit(10).unwrap() as usize]);
                is_space = false;
            }
        }

        let mut done = false;
        let mut skip = 0;

        //for (i, &d) in disk.iter().enumerate() {
        for mut i in (0..disk.len()).rev() {
            if skip > 0 {
                skip -= 1;
            } else {
                if disk[i].is_some() {
                    let mut size: usize = 1;

                    for j in (0..i).rev() {
                        if disk[j].is_some() && disk[j].unwrap() == disk[i].unwrap() {
                            size += 1;
                        } else {
                            break;
                        }
                    }

                    skip = size-1;

                    let mut space_size = 0;
                    for j in 0..i {
                        if done {
                            break;
                        }
                        if disk[j].is_none() {
                            space_size += 1;

                            if space_size == size {
                                println!("{i}");
                                for k in 0..space_size {
                                    disk.swap(j - k, i - k);
                                    /*println!("{}", disk.iter().map(|x| {
                                        if x.is_some() {
                                            x.unwrap().to_string()
                                        } else {
                                            ".".parse().unwrap()
                                        }
                                    }).collect::<String>());*/
                                }
                                done = true;
                                break;
                            }
                        } else {
                            space_size = 0;
                        }
                    }
                    done = false;
                }
            }


        }

        /*println!("{}", disk.iter().map(|x| {
            if x.is_some() {
                x.unwrap().to_string()
            } else {
                ".".parse().unwrap()
            }
        }).collect::<String>());*/

        let answer: i64 = disk.iter().enumerate().map(|(i, &id)| {
            (i as i64) * id.unwrap_or(0) as i64
        }).sum();

        Ok(answer as usize)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

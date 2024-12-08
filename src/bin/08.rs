use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use gcd::Gcd;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn parse_input<R: BufRead>(reader: &mut R, map: &mut Vec<Vec<char>>) {
        *map = reader.lines().map(|x| {
            x.unwrap().chars().collect()
        }).collect();
    }

    fn print_map(map: &Vec<Vec<char>>) {
        println!();
        map.iter().for_each(|x| {
            println!("  {}", x.iter().collect::<String>());
        });
        println!();
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        parse_input(&mut reader, &mut map);

        let height = map.len() as i32;
        let width = map[0].len() as i32;

        let mut answer = 0;

        let mut map_antinodes = map.clone();

        for a_y in 0..height {
            for a_x in 0..width {
                let a = map[a_y as usize][a_x as usize];

                if a != '.' && a != '#' {
                    for b_y in 0..height {
                        for b_x in 0..width {
                            if !(b_x == a_x && b_y == a_y) {
                                let b = map[b_y as usize][b_x as usize];

                                if b == a {
                                    let d_x = 2*a_x - b_x;
                                    let d_y = 2*a_y - b_y;

                                    if d_x >= 0 && d_y >= 0
                                        && d_x < width && d_y < height
                                        && map_antinodes[d_y as usize][d_x as usize] != '#'
                                    {
                                        map_antinodes[d_y as usize][d_x as usize] = '#';
                                        answer += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        //print_map(&map_antinodes);

        Ok(answer as usize)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        parse_input(&mut reader, &mut map);

        let height = map.len() as i32;
        let width = map[0].len() as i32;

        let mut answer = 0;

        let mut map_antinodes = map.clone();

        for a_y in 0..height {
            for a_x in 0..width {
                let a = map[a_y as usize][a_x as usize];

                if a != '.' && a != '#' {
                    for b_y in 0..height {
                        for b_x in 0..width {
                            if !(b_x == a_x && b_y == a_y) {
                                let b = map[b_y as usize][b_x as usize];

                                if b == a {
                                    let mut d_x = a_x - b_x;
                                    let mut d_y = a_y - b_y;

                                    let gcd = (d_x.abs() as u32).gcd(d_y.abs() as u32) as i32;

                                    d_x = d_x / gcd;
                                    d_y = d_y / gcd;

                                    let (mut c_x, mut c_y) = (a_x, a_y);

                                    while c_x >= 0 && c_y >= 0 && c_x < width && c_y < height {
                                        if map_antinodes[c_y as usize][c_x as usize] != '#' {
                                            map_antinodes[c_y as usize][c_x as usize] = '#';
                                            answer += 1;
                                        }

                                        c_x += d_x;
                                        c_y += d_y;
                                    }

                                    c_x = a_x;
                                    c_y = a_y;

                                    while c_x >= 0 && c_y >= 0 && c_x < width && c_y < height {
                                        if map_antinodes[c_y as usize][c_x as usize] != '#' {
                                            map_antinodes[c_y as usize][c_x as usize] = '#';
                                            answer += 1;
                                        }

                                        c_x -= d_x;
                                        c_y -= d_y;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        //print_map(&map_antinodes);

        Ok(answer as usize)
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

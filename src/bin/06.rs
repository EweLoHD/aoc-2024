use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
fn main() -> Result<()> {
    start_day(DAY);

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

    fn find_guard_starting_pos(map: &Vec<Vec<char>>) -> Result<(i32, i32), Error>{
        for y in 0..map.len() {
            let l = &map[y];
            for x in 0..l.len() {
                if l[x] == '^' {
                    return Ok((x as i32, y as i32))
                }
            }
        }

        Err(Error::msg("Guard not found"))
    }

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        parse_input(&mut reader, &mut map);

        let height = map.len() as i32;
        let width = map[0].len() as i32;

        //print_map(&map);

        let (mut x, mut y) = find_guard_starting_pos(&map)?;
        let mut answer: usize = 0;

        while x < width && y < height {
            let c = map[y as usize][x as usize];

            let (x_d, y_d): (i32, i32);

            let next_dir;

            if c == '^' {
                x_d = 0;
                y_d = -1;
                next_dir = '>';
            } else if c == 'v' {
                x_d = 0;
                y_d = 1;
                next_dir = '<';
            } else if c == '<' {
                x_d = -1;
                y_d = 0;
                next_dir = '^';
            } else if c == '>' {
                x_d = 1;
                y_d = 0;
                next_dir = 'v';
            } else {
                panic!("No guard at current pos")
            }

            if y + y_d >= height || x + x_d >= width || y + y_d < 0 || x + x_d < 0 {
                break;
            }

            if map[(y + y_d) as usize][(x + x_d) as usize] == '#' {
                map[y as usize][x as usize] = next_dir;
            } else {
                map[y as usize][x as usize] = 'X';

                x += x_d;
                y += y_d;

                if map[y as usize][x as usize] == '.' {
                    answer += 1;
                }

                map[y as usize][x as usize] = c;

            }
        }

        //print_map(&map);

        Ok(answer+1)
    }


    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn is_stuck(map: &mut Vec<Vec<char>>, mut x: i32, mut y: i32, width: i32, height: i32) -> bool {
        //let mut prev: Option<(char, i32, i32)> = None;
        let mut next: char = '^';
        let mut prev: char = '.';

        while x < width && y < height {
            let c = map[y as usize][x as usize];

            let (x_d, y_d): (i32, i32);

            let next_dir;

            if c == '^' {
                x_d = 0;
                y_d = -1;
                next_dir = '>';
            } else if c == 'v' {
                x_d = 0;
                y_d = 1;
                next_dir = '<';
            } else if c == '<' {
                x_d = -1;
                y_d = 0;
                next_dir = '^';
            } else if c == '>' {
                x_d = 1;
                y_d = 0;
                next_dir = 'v';
            } else {
                panic!("No guard at current pos")
            }

            if y + y_d >= height || x + x_d >= width || y + y_d < 0 || x + x_d < 0 {
                break;
            }

            if map[(y + y_d) as usize][(x + x_d) as usize] == '#' || map[(y + y_d) as usize][(x + x_d) as usize] == '%' {
                if prev == '+' && map[(y + y_d) as usize][(x + x_d) as usize] == '%' {
                    //print_map(map);
                    return true;
                }

                // Rotate
                map[y as usize][x as usize] = next_dir;
                next = '+';

                map[(y + y_d) as usize][(x + x_d) as usize] = '%'
            } else {
                map[y as usize][x as usize] = next;

                if x_d == 0 {
                    next = '|';
                } else {
                    next = '-';
                }

                x += x_d;
                y += y_d;

                /*if map[y as usize][x as usize] == '.' {
                    answer += 1;
                }*/

                if map[y as usize][x as usize] == '|' || map[y as usize][x as usize] == '-' {
                    next = '+';
                }

                prev = map[y as usize][x as usize];

                map[y as usize][x as usize] = c;

            }
        }

        //print_map(map);

        false
    }

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        parse_input(&mut reader, &mut map);

        let height = map.len() as i32;
        let width = map[0].len() as i32;

        //print_map(&map);

        let mut answer = 0 as usize;

        let (x, y) = find_guard_starting_pos(&map)?;

        for y_o in 0..height {
            for x_o in 0..width {
                //println!("{} {}", x_o, y_o);
                if !(y_o == y && x_o == x) && map[y_o as usize][x_o as usize] != '#' {
                    let mut n_map = map.clone();
                    n_map[y_o as usize][x_o as usize] = '#';

                    if is_stuck(&mut n_map, x, y, width, height) {
                        n_map[y_o as usize][x_o as usize] = 'O';
                        //print_map(&n_map);
                        answer += 1;
                    }
                }
            }
        }

        //let stuck = is_stuck(&mut map, x, y, width, height);

        //println!("stuck: {}", stuck);

        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

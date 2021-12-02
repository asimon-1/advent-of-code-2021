use std::{error::Error, fs};
use regex::Regex;

fn read_file(filename: &str) -> String {
    println!("In file {}", filename);
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "input/input.txt";
    let contents = read_file(filename);
    println!("Part 1: {}", part_1(&contents)?);
    println!("Part 2: {}", part_2(&contents)?);
    Ok(())
}

fn part_1(input: &str) -> Result<u32, Box<dyn Error>> {
    let lines: Vec<&str> = input
        .lines()
        .collect();

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;

    let horiz_pattern = Regex::new(r"forward (\d+)").unwrap();
    let down_pattern = Regex::new(r"down (\d+)").unwrap();
    let up_pattern = Regex::new(r"up (\d+)").unwrap();
    for line in lines {
        // println!("Line: {}", line);
        match horiz_pattern.captures(line).and_then(|captured| captured.get(1)) {
            Some(val) => {
                // println!("Matched horizontal pattern with val: {:?}", val);
                horizontal += val.as_str().parse::<u32>().unwrap();
            },
            None => {
                match down_pattern.captures(line).and_then(|captured| captured.get(1)) {
                    Some(val) => {
                        // println!("Matched down pattern with val: {:?}", val);
                        depth += val.as_str().parse::<u32>().unwrap()
                    },
                    None => {
                        match up_pattern.captures(line).and_then(|captured| captured.get(1)) {
                            Some(val) => {
                                // println!("Matched up pattern with val: {:?}", val);
                                depth -= val.as_str().parse::<u32>().unwrap()
                            },
                            None => (),
                        }
                    }
                }
            }
        }
    }
    Ok(horizontal * depth)
}

#[test]
fn test_part1() {
    let filename = "input/test.txt";
    let fn_val = part_1(&read_file(filename)).unwrap();
    let test_val = 150;
    assert_eq!(fn_val, test_val);
}

fn part_2(input: &str) -> Result<i32, Box<dyn Error>> {
    let lines: Vec<&str> = input
        .lines()
        .collect();

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    let horiz_pattern = Regex::new(r"forward (\d+)").unwrap();
    let down_pattern = Regex::new(r"down (\d+)").unwrap();
    let up_pattern = Regex::new(r"up (\d+)").unwrap();
    for line in lines {
        // println!("Line: {}", line);
        match horiz_pattern.captures(line).and_then(|captured| captured.get(1)) {
            Some(val) => {
                // println!("Matched horizontal pattern with val: {:?}", val);
                let val = val.as_str().parse::<i32>().unwrap();
                horizontal += val;
                depth += val * aim;
            },
            None => {
                match down_pattern.captures(line).and_then(|captured| captured.get(1)) {
                    Some(val) => {
                        // println!("Matched down pattern with val: {:?}", val);
                        aim += val.as_str().parse::<i32>().unwrap();
                    },
                    None => {
                        match up_pattern.captures(line).and_then(|captured| captured.get(1)) {
                            Some(val) => {
                                // println!("Matched up pattern with val: {:?}", val);
                                aim -= val.as_str().parse::<i32>().unwrap();
                            },
                            None => (),
                        }
                    }
                }
            }
        }
    }
    Ok(horizontal * depth)
}

#[test]
fn test_part2() {
    let filename = "input/test.txt";
    let fn_val = part_2(&read_file(filename)).unwrap();
    let test_val: i32 = 900;
    assert_eq!(fn_val, test_val);
}

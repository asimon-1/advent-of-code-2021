use std::{error::Error, fs};

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
    let mut count: u32 = 0;
    let mut last_val: u32 = 999999999;
    for line in input.lines() {
        let current_val: u32 = line.parse()?;
        if current_val > last_val {
            count += 1
        }
        last_val = current_val
    }
    Ok(count)
}

#[test]
fn test_part1() {
    let filename = "input/test.txt";
    let fn_val = part_1(&read_file(filename)).unwrap();
    let test_val = 7;
    assert_eq!(fn_val, test_val);
}

fn part_2(input: &str) -> Result<u32, Box<dyn Error>> {
    let lines: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().expect("Could not parse the value"))
        .collect();
    let slices = lines.windows(3);

    let mut count: u32 = 0;
    let mut last_sum: u32 = 999999999;
    for slice in slices {
        let current_sum: u32 = slice.iter().sum();
        if current_sum > last_sum {
            count += 1
        }
        last_sum = current_sum;
    }
    Ok(count)
}

#[test]
fn test_part2() {
    let filename = "input/test.txt";
    let fn_val = part_2(&read_file(filename)).unwrap();
    let test_val = 5;
    assert_eq!(fn_val, test_val);
}

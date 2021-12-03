use std::{error::Error, fs};

fn read_file(filename: &str) -> String {
    println!("In file {}", filename);
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn to_u32(slice: &[u32]) -> u32 {
    // Manually convert binary number in a vector slice of bits to a decimal integer
    slice
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + x * u32::pow(2, i as u32))
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "input/input.txt";
    let contents = read_file(filename);
    println!("Part 1: {}", part_1(&contents)?);
    println!("Part 2: {}", part_2(&contents)?);
    Ok(())
}

fn part_1(input: &str) -> Result<u32, Box<dyn Error>> {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect();

    let line_len = lines[0].len();

    // Since there are only two options, if 1 is the most common, then the sum will be greater than half the total
    // If 0 is the most common then the sum will be less than half the total
    let threshold: u32 = (lines.len() / 2) as u32;

    // Count the number of 1's in each column
    let sums = lines.iter().fold(vec![0; line_len], |mut acc, line| {
        line.iter().enumerate().for_each(|(i, col)| acc[i] += col);
        acc
    });

    // Compare to the threshold to determine digits.
    // Note that integer division rounds down, so
    // x > threshold --> x / threshold == 1
    // x < threshold --> x / threshold == 0
    let gamma_digits: Vec<u32> = sums.iter().map(|x| x / threshold).collect();

    // Invert gamma to determine epsilon
    let epsilon_digits: Vec<u32> = gamma_digits
        .iter()
        .map(|x| match x {
            0 => 1,
            1 => 0,
            _ => 0,
        })
        .collect();
    Ok(to_u32(&gamma_digits) * to_u32(&epsilon_digits))
}

#[test]
fn test_part1() {
    let filename = "input/test.txt";
    let fn_val = part_1(&read_file(filename)).unwrap();
    let test_val = 198;
    assert_eq!(fn_val, test_val);
}

fn part_2(input: &str) -> Result<i32, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();

    Ok(0)
}

#[test]
fn test_part2() {
    let filename = "input/test.txt";
    let fn_val = part_2(&read_file(filename)).unwrap();
    let test_val: i32 = 0;
    assert_eq!(fn_val, test_val);
}

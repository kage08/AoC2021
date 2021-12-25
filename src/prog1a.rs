use std::fs;

pub fn main1() {
    let numbers = parsefile("inputs/input1a.txt");
    let mut inc = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            inc += 1;
        }
    }
    println!("{}", inc);
}

pub fn main2() {
    let numbers = parsefile("inputs/input1a.txt");
    let mut inc = 0;
    let mut sum = numbers[0] + numbers[1] + numbers[2];
    for i in 1..(numbers.len() - 2) {
        let new_sum = sum + numbers[i + 2] - numbers[i - 1];
        if new_sum > sum {
            inc += 1;
        }
        sum = new_sum;
    }
    println!("{}", inc);
}

fn parsefile(filename: &str) -> Vec<i32> {
    let lines = fs::read_to_string(filename).expect("Unable to read file");
    let numbers = lines
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    numbers
}

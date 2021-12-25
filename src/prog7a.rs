use crate::utils::parsefile;

pub fn main1() {
    let lines = parsefile("inputs/input7a.txt");
    let mut input = lines[0]
        .split(",")
        .map(|nu| nu.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    input.sort();
    let pos = input[input.len() / 2];
    let fuel: u64 = input.iter().map(|n| ((n - pos).abs() as u64)).sum();
    println!("{}", fuel);
}

fn get_cost(arr: &Vec<i32>, pos: i32) -> u64 {
    let ans: u64 = arr
        .iter()
        .map(|n| {
            let diff = (n - pos).abs();
            ((diff * (diff + 1)) / 2) as u64
        })
        .sum();
    ans
}

pub fn main2() {
    let lines = parsefile("inputs/input7a.txt");
    let mut input = lines[0]
        .split(",")
        .map(|nu| nu.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    input.sort();
    let min_cost = (input[0]..(input[input.len() - 1]))
        .map(|n| get_cost(&input, n))
        .min()
        .unwrap();
    println!("{}", min_cost);
}

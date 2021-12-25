use crate::utils::parsefile;

pub fn main1() {
    let lines = parsefile("inputs/input2a.txt");
    let mut posx = 0;
    let mut posy = 0;
    for line in lines {
        let words = line
            .split(" ")
            .filter(|word| word.len() > 0)
            .collect::<Vec<&str>>();
        match words[0] {
            "forward" => {
                posx += words[1].parse::<i32>().unwrap();
            }
            "up" => {
                posy -= words[1].parse::<i32>().unwrap();
            }
            "down" => {
                posy += words[1].parse::<i32>().unwrap();
            }
            _ => {
                panic!("Unknown command: {}", words[0]);
            }
        }
    }
    println!("{}", posx * posy);
}

pub fn main2() {
    let lines = parsefile("inputs/input2a.txt");
    let mut posx = 0;
    let mut posy = 0;
    let mut aim = 0;
    for line in lines {
        let words = line
            .split(" ")
            .filter(|word| word.len() > 0)
            .collect::<Vec<&str>>();
        match words[0] {
            "forward" => {
                let x = words[1].parse::<i32>().unwrap();
                posx += x;
                posy += aim * x;
            }
            "up" => {
                aim -= words[1].parse::<i32>().unwrap();
            }
            "down" => {
                aim += words[1].parse::<i32>().unwrap();
            }
            _ => {
                panic!("Unknown command: {}", words[0]);
            }
        }
    }
    println!("{}", posx * posy);
}

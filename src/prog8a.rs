use crate::utils::parsefile;

pub fn main1() {
    let lines = parsefile("inputs/input8a.txt");
    let ans: u32 = lines
        .iter()
        .map(|s| s.split(" | ").collect::<Vec<&str>>()[1])
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            s.split(" ")
                .map(|t| {
                    if [2, 3, 4, 7].contains(&(t.len())) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum();

    println!("{}", ans);
}

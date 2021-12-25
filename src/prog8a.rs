use crate::utils::parsefile;
use std::collections::HashMap;

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

fn segment_to_num(st: &str) -> u32 {
    let mut ans: u32 = 0;
    assert!(st.chars().all(|c| c.is_alphabetic()));
    for c in st.chars() {
        ans += 1 << (c as u32 - 'a' as u32);
    }
    ans
}

fn contains_within(a: &u32, b: &u32) -> bool {
    *a == (a & b)
}

fn map_segments(examples: &Vec<&str>) -> HashMap<u32, u32> {
    let num_arr = examples
        .iter()
        .map(|s| segment_to_num(s))
        .collect::<Vec<u32>>();

    let mut ans: HashMap<u32, u32> = HashMap::new();

    ans.insert(
        1,
        num_arr
            .iter()
            .filter(|n| n.count_ones() == 2)
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0] as u32,
    );

    ans.insert(
        7,
        num_arr
            .iter()
            .filter(|n| n.count_ones() == 3)
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0] as u32,
    );
    ans.insert(
        4,
        num_arr
            .iter()
            .filter(|n| n.count_ones() == 4)
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0] as u32,
    );
    ans.insert(
        8,
        num_arr
            .iter()
            .filter(|n| n.count_ones() == 7)
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0] as u32,
    );

    ans.insert(
        9,
        num_arr
            .iter()
            .filter(|n| (n.count_ones() == 6) && contains_within(&ans[&4], n))
            .collect::<Vec<&u32>>()[0]
            .clone(),
    );


    ans.insert(
        0,
        num_arr
            .iter()
            .filter(|n| (n.count_ones() == 6) && (**n != ans[&9]) && contains_within(&ans[&7], *n))
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0],
    );

    ans.insert(
        6,
        num_arr
            .iter()
            .filter(|n| (n.count_ones() == 6) && (**n != ans[&9]) && (**n != ans[&0]))
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0],
    );

    ans.insert(
        5,
        num_arr
            .iter()
            .filter(|n| (n.count_ones() == 5) && contains_within(*n, &ans[&6]))
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0],
    );

    ans.insert(
        3,
        num_arr
            .iter()
            .filter(|n| (n.count_ones() == 5) && (**n != ans[&5]) && contains_within(*n, &ans[&9]))
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0],
    );

    ans.insert(
        2,
        num_arr
            .iter()
            .filter(|n| (n.count_ones() == 5) && (**n != ans[&5]) && (**n != ans[&3]))
            .map(|n| n.clone())
            .collect::<Vec<u32>>()[0],
    );

    ans
}

fn my_reverse_map(mp: &HashMap<u32, u32>) -> HashMap<u32, u32> {
    let mut ans: HashMap<u32, u32> = HashMap::new();
    for (k, v) in mp {
        ans.insert(v.clone(), k.clone());
    }
    ans
}

pub fn main2() {
    let lines = parsefile("inputs/input8a.txt");
    let hints = lines
        .iter()
        .map(|s| s.split(" | ").collect::<Vec<&str>>()[0])
        .collect::<Vec<&str>>();

    let queries = lines
        .iter()
        .map(|s| s.split(" | ").collect::<Vec<&str>>()[1])
        .collect::<Vec<&str>>();

    let mut ans_vec: Vec<u32> = Vec::new();

    for (i, ht) in hints.iter().enumerate() {
        let mp = map_segments(&ht.split(" ").collect::<Vec<&str>>());
        let rev_mp = my_reverse_map(&mp);
        let num_ans = queries[i]
            .split(" ")
            .map(|s| segment_to_num(s))
            .map(|n| rev_mp[&n])
            .collect::<Vec<u32>>();
        let mut this_ans: u32 = 0;
        for (j, n) in num_ans.iter().rev().enumerate() {
            this_ans += n * (10 as u32).pow(j as u32);
        }
        ans_vec.push(this_ans);
    }

    println!("{}", ans_vec.iter().sum::<u32>());
}

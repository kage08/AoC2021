use crate::utils::parsefile;

pub fn main1() {
    let lines = parsefile("inputs/input3a.txt");
    let num_lines = lines.len() as i32;
    let mut arr = vec![0 as i32; lines[0].len()];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                arr[i] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, n) in arr.iter().rev().enumerate() {
        if n * 2 > num_lines {
            gamma += 2_i32.pow(i as u32);
        } else {
            epsilon += 2_i32.pow(i as u32);
        }
    }
    println!("{}", gamma * epsilon);
}

fn find_most(arr: &Vec<String>, idx: &Vec<usize>, place: usize, collect_most: bool) -> Vec<usize> {
    let mut num1 = 0;
    for i in idx {
        if arr[*i].chars().nth(place).unwrap() == '1' {
            num1 += 1;
        }
    }

    let bit_most = if 2 * num1 >= idx.len() { '1' } else { '0' };

    let ans_vec = if collect_most {
        idx.iter()
            .filter(|i| arr[**i].chars().nth(place).unwrap() == bit_most)
            .map(|i| *i as usize)
            .collect::<Vec<usize>>()
    } else {
        idx.iter()
            .filter(|i| arr[**i].chars().nth(place).unwrap() != bit_most)
            .map(|i| *i as usize)
            .collect::<Vec<usize>>()
    };
    ans_vec
}

fn binstr_to_dec(binstr: &str) -> i32 {
    let mut dec = 0;
    for (i, c) in binstr.chars().rev().enumerate() {
        if c == '1' {
            dec += 2_i32.pow(i as u32);
        }
    }
    dec
}

pub fn main2() {
    let lines = parsefile("inputs/input3a.txt");
    let num_lines = lines.len();
    let mut oidx = (0..num_lines).collect::<Vec<usize>>();
    let mut place = 0;
    while (oidx.len() > 1) && (place < lines[0].len()) {
        oidx = find_most(&lines, &oidx, place, true);
        place += 1;
    }
    let oans = binstr_to_dec(&lines[oidx[0]]);

    let mut cidx = (0..num_lines).collect::<Vec<usize>>();
    let mut place = 0;
    while (cidx.len() > 1) && (place < lines[0].len()) {
        cidx = find_most(&lines, &cidx, place, false);
        place += 1;
    }
    let cans = binstr_to_dec(&lines[cidx[0]]);

    println!("{}", oans * cans);
}

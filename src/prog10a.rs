use crate::utils::parsefile;

fn compute_score(ln: &String) -> u32 {
    let mut stk: Vec<char> = Vec::new();
    for c in ln.chars() {
        match c {
            '(' | '[' | '{' | '<' => stk.push(c),
            ')' => {
                if !stk.is_empty() && *stk.last().unwrap() == '(' {
                    stk.pop();
                } else {
                    return 3u32;
                }
            }
            ']' => {
                if !stk.is_empty() && *stk.last().unwrap() == '[' {
                    stk.pop();
                } else {
                    return 57u32;
                }
            }
            '}' => {
                if !stk.is_empty() && *stk.last().unwrap() == '{' {
                    stk.pop();
                } else {
                    return 1197u32;
                }
            }
            '>' => {
                if !stk.is_empty() && *stk.last().unwrap() == '<' {
                    stk.pop();
                } else {
                    return 25137u32;
                }
            }
            _ => panic!("Wrong char: {}", c),
        }
    }
    0
}

pub fn main1() {
    let lines = parsefile("inputs/input10a.txt");
    let ans: u64 = lines.iter().map(|s| compute_score(s) as u64).sum();
    println!("{}", ans);
}

fn compute_score2(ln: &String) -> u64 {
    let mut stk: Vec<char> = Vec::new();
    for c in ln.chars() {
        match c {
            '(' | '[' | '{' | '<' => stk.push(c),
            ')' => {
                if !stk.is_empty() && *stk.last().unwrap() == '(' {
                    stk.pop();
                } else {
                    return 0;
                }
            }
            ']' => {
                if !stk.is_empty() && *stk.last().unwrap() == '[' {
                    stk.pop();
                } else {
                    return 0;
                }
            }
            '}' => {
                if !stk.is_empty() && *stk.last().unwrap() == '{' {
                    stk.pop();
                } else {
                    return 0;
                }
            }
            '>' => {
                if !stk.is_empty() && *stk.last().unwrap() == '<' {
                    stk.pop();
                } else {
                    return 0;
                }
            }
            _ => panic!("Wrong char: {}", c),
        }
    }
    let mut score: u64 = 0;
    for c in stk.iter().rev() {
        score = score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
    }
    score
}

pub fn main2() {
    let lines = parsefile("inputs/input10a.txt");
    let mut ans = lines
        .iter()
        .map(|s| compute_score2(s))
        .filter(|n| *n > 0)
        .collect::<Vec<u64>>();
    ans.sort();
    println!("{}", ans[ans.len() / 2]);
}

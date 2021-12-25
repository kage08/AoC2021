use crate::utils::parsefile;

fn update_vec1(inp: &Vec<u64>) -> Vec<u64> {
    let mut ans: Vec<u64> = vec![0; 10];
    for i in 1..10 {
        ans[i - 1] = inp[i];
    }
    ans[6] += inp[0];
    ans[8] += inp[0];
    return ans;
}

pub fn main1() {
    let lines = parsefile("inputs/input6a.txt");
    let input = lines[0]
        .split(",")
        .map(|nu| nu.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut ans: Vec<u64> = vec![0; 10];
    for n in input {
        ans[n as usize] += 1;
    }

    for _ in 0..256 {
        ans = update_vec1(&ans);
    }

    let mut sum: u64 = 0;
    for n in ans {
        sum += n as u64;
    }
    println!("{}", sum);
}

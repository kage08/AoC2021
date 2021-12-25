use std::collections::HashSet;

use crate::utils::parsefile;

fn is_low(board: &Vec<Vec<u32>>, w: u32, h: u32, r: u32, c: u32) -> bool {
    if (r > 0u32) && (board[r as usize][c as usize] >= board[(r - 1) as usize][c as usize]) {
        return false;
    }
    if (c > 0u32) && (board[r as usize][c as usize] >= board[r as usize][(c - 1) as usize]) {
        return false;
    }
    if (r < h - 1) && (board[r as usize][c as usize] >= board[(r + 1) as usize][c as usize]) {
        return false;
    }
    if (c < w - 1) && (board[r as usize][c as usize] >= board[r as usize][(c + 1) as usize]) {
        return false;
    }
    true
}

pub fn main1() {
    let lines = parsefile("inputs/input9a.txt");
    let mut board: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut line_vec: Vec<u32> = Vec::new();
        for c in line.chars() {
            line_vec.push(c as u32 - '0' as u32);
        }
        board.push(line_vec);
    }
    let width = board[0].len() as u32;
    let height = board.len() as u32;

    let mut ans: u32 = 0;

    for i in 0..height {
        for j in 0..width {
            if is_low(&board, width, height, i, j) {
                ans += board[i as usize][j as usize] + 1;
            }
        }
    }

    println!("{}", ans);
}

fn coord_to_num(w: u32, r: u32, c: u32) -> u32 {
    (r * w) + c
}

fn update_state(board: &Vec<Vec<u32>>, w: u32, h: u32, r: u32, c: u32, visited: &mut HashSet<u32>) {
    let val = board[r as usize][c as usize];
    if val == 9 {
        return;
    }
    visited.insert(coord_to_num(w, r, c));

    if (r > 0u32)
        && !(visited.contains(&coord_to_num(w, r - 1, c)))
        && (board[r as usize][c as usize] < board[(r - 1) as usize][c as usize])
    {
        update_state(board, w, h, r - 1, c, visited);
    }
    if (c > 0u32)
        && !(visited.contains(&coord_to_num(w, r, c - 1)))
        && (board[r as usize][c as usize] < board[r as usize][(c - 1) as usize])
    {
        update_state(board, w, h, r, c - 1, visited);
    }
    if (r < h - 1)
        && !(visited.contains(&coord_to_num(w, r + 1, c)))
        && (board[r as usize][c as usize] < board[(r + 1) as usize][c as usize])
    {
        update_state(board, w, h, r + 1, c, visited);
    }
    if (c < w - 1)
        && !(visited.contains(&coord_to_num(w, r, c + 1)))
        && (board[r as usize][c as usize] < board[r as usize][(c + 1) as usize])
    {
        update_state(board, w, h, r, c + 1, visited);
    }
}

pub fn main2() {
    let lines = parsefile("inputs/input9a.txt");
    let mut board: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut line_vec: Vec<u32> = Vec::new();
        for c in line.chars() {
            line_vec.push(c as u32 - '0' as u32);
        }
        board.push(line_vec);
    }
    let width = board[0].len() as u32;
    let height = board.len() as u32;

    let mut sizes: Vec<u32> = Vec::new();

    for i in 0..height {
        for j in 0..width {
            if is_low(&board, width, height, i, j) {
                let mut visited: HashSet<u32> = HashSet::new();
                update_state(&board, width, height, i, j, &mut visited);
                sizes.push(visited.len() as u32);
            }
        }
    }

    sizes.sort();
    sizes.reverse();
    println!("{}", sizes[0] * sizes[1] * sizes[2]);
}

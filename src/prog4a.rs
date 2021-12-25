use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Entry {
    row: Vec<usize>,
    col: Vec<usize>,
}
struct Board {
    entrydict: HashMap<i32, Entry>,
    unmarked: HashSet<i32>,
    sum_rows: Vec<usize>,
    sum_cols: Vec<usize>,
}

impl Board {
    fn new() -> Board {
        Board {
            entrydict: HashMap::new(),
            unmarked: HashSet::new(),
            sum_rows: vec![0; 5],
            sum_cols: vec![0; 5],
        }
    }

    fn add_value(&mut self, n: i32, r: usize, c: usize) {
        if self.entrydict.contains_key(&n) {
            self.entrydict.get_mut(&n).unwrap().add_loc(r, c);
        } else {
            self.entrydict.insert(n, Entry::new(r, c));
        }
        self.unmarked.insert(n);
    }

    fn add_mark(&mut self, n: i32) {
        if !self.unmarked.contains(&n) {
            return;
        }
        self.unmarked.remove(&n);
        let entry = &self.entrydict[&n];
        for i in 0..entry.row.len() {
            self.sum_rows[entry.row[i]] += 1;
            self.sum_cols[entry.col[i]] += 1;
        }
    }

    fn get_marked_sum(&self) -> i32 {
        let mut sum = 0;
        for i in self.unmarked.iter() {
            sum += i * self.entrydict[i].row.len() as i32;
        }
        sum
    }

    fn check_marked(&self) -> bool {
        (self.sum_cols.contains(&5)) || (self.sum_rows.contains(&5))
    }
}

impl Entry {
    fn new(row: usize, col: usize) -> Entry {
        Entry {
            row: vec![row],
            col: vec![col],
        }
    }

    fn add_loc(&mut self, r: usize, c: usize) {
        self.row.push(r);
        self.col.push(c);
    }
}

fn parsefile(filename: &str) -> Vec<String> {
    let lines = fs::read_to_string(filename).expect("Unable to read file");
    let linevec = lines
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    linevec
}

pub fn main1() {
    let lines = parsefile("inputs/input4a.txt");
    let mut l_num = 0 as usize;
    let tot_lines = lines.len();
    let mark_num = lines[0]
        .split(",")
        .filter(|num| num.len() > 0)
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    l_num += 1;

    let mut boards: Vec<Board> = vec![];
    let mut bcount = 0 as usize;

    while l_num < tot_lines {
        boards.push(Board::new());
        bcount += 1;
        for rw in 0..5 {
            let nums = lines[l_num]
                .split(" ")
                .filter(|num| num.len() > 0)
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            for (cl, n) in nums.iter().enumerate() {
                boards[bcount - 1].add_value(*n, rw, cl);
            }
            l_num += 1;
        }
    }

    let mut m_idx = 0 as usize;
    let mark_len = mark_num.len();

    while m_idx < mark_len {
        let curr_num = mark_num[m_idx];
        for b in boards.iter_mut() {
            b.add_mark(curr_num);
            if b.check_marked() {
                let t1 = b.get_marked_sum();
                println!("{}", t1 * curr_num);
                return;
            }
        }
        m_idx += 1;
    }
}

pub fn main2() {
    let lines = parsefile("inputs/input4a.txt");
    let mut l_num = 0 as usize;
    let tot_lines = lines.len();
    let mark_num = lines[0]
        .split(",")
        .filter(|num| num.len() > 0)
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    l_num += 1;

    let mut boards: Vec<Board> = vec![];
    let mut bcount = 0 as usize;

    while l_num < tot_lines {
        boards.push(Board::new());
        bcount += 1;
        for rw in 0..5 {
            let nums = lines[l_num]
                .split(" ")
                .filter(|num| num.len() > 0)
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            for (cl, n) in nums.iter().enumerate() {
                boards[bcount - 1].add_value(*n, rw, cl);
            }
            l_num += 1;
        }
    }

    let mut left_boards: HashSet<usize> = (0..boards.len()).collect();

    let mut m_idx = 0 as usize;
    let mark_len = mark_num.len();

    while m_idx < mark_len {
        let curr_num = mark_num[m_idx];
        let curr_left = left_boards.clone();
        for i in curr_left {
            let b = &mut boards[i];
            b.add_mark(curr_num);
            if b.check_marked() {
                left_boards.remove(&i);
                if left_boards.is_empty() {
                    let t1 = b.get_marked_sum();
                    println!("{}", t1 * curr_num);
                    return;
                }
            }
        }
        m_idx += 1;
    }
}

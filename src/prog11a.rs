use crate::utils::parsefile;

struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Coords {
        Coords { x: x, y: y }
    }
}

fn update_board(board: &mut Vec<Vec<u32>>, height: i32, width: i32) -> u32 {
    let mut flashes: u32 = 0;
    let mut to_explode: Vec<Coords> = Vec::new();

    fn update_cell(
        x: i32,
        y: i32,
        to_explode: &mut Vec<Coords>,
        board: &mut Vec<Vec<u32>>,
        height: i32,
        width: i32,
        strict: bool,
    ) {
        if (x < 0) || (x >= height) || (y < 0) || (y >= width) {
            return;
        }
        if !strict && (board[x as usize][y as usize] == 0) {
            return;
        }
        board[x as usize][y as usize] += 1;
        if board[x as usize][y as usize] > 9 {
            to_explode.push(Coords::new(x, y));
        }
    }

    for i in 0..height {
        for j in 0..width {
            update_cell(i, j, &mut to_explode, board, height, width, true);
        }
    }

    while !to_explode.is_empty() {
        let crd = to_explode.pop().unwrap();
        if board[crd.x as usize][crd.y as usize] == 0 {
            continue;
        }
        board[crd.x as usize][crd.y as usize] = 0;
        update_cell(
            crd.x + 1,
            crd.y,
            &mut to_explode,
            board,
            height,
            width,
            false,
        );
        update_cell(
            crd.x - 1,
            crd.y,
            &mut to_explode,
            board,
            height,
            width,
            false,
        );
        update_cell(
            crd.x,
            crd.y + 1,
            &mut to_explode,
            board,
            height,
            width,
            false,
        );
        update_cell(
            crd.x,
            crd.y - 1,
            &mut to_explode,
            board,
            height,
            width,
            false,
        );
        update_cell(
            crd.x + 1,
            crd.y + 1,
            &mut to_explode,
            board,
            height,
            width,
            false,
        );
        update_cell(
            crd.x + 1,
            crd.y - 1,
            &mut to_explode,
            board,
            height,
            width,
            false,
        );
        update_cell(
            crd.x - 1,
            crd.y + 1,
            &mut to_explode,
            board,
            height,
            width,
            false,
        );
        update_cell(
            crd.x - 1,
            crd.y - 1,
            &mut to_explode,
            board,
            height,
            width,
            false,
        );
        flashes += 1;
    }

    flashes
}

pub fn main1() {
    let lines = parsefile("inputs/input11a.txt");
    let mut board: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut this_vec: Vec<u32> = Vec::new();
        for c in line.chars() {
            if !c.is_alphanumeric() {
                panic!("{} is not numeric", c);
            }
            this_vec.push(c as u32 - '0' as u32);
        }
        board.push(this_vec);
    }
    let mut tot_flashes: u32 = 0;
    let (height, width) = (board.len(), board[0].len());

    for _ in 0..100 {
        tot_flashes += update_board(&mut board, height as i32, width as i32);
    }

    println!("{}", tot_flashes);
}

pub fn main2() {
    let lines = parsefile("inputs/input11a.txt");
    let mut board: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut this_vec: Vec<u32> = Vec::new();
        for c in line.chars() {
            if !c.is_alphanumeric() {
                panic!("{} is not numeric", c);
            }
            this_vec.push(c as u32 - '0' as u32);
        }
        board.push(this_vec);
    }
    let (height, width) = (board.len(), board[0].len());

    for t in 0.. {
        if update_board(&mut board, height as i32, width as i32) >= ((height * width) as u32) {
            println!("{}", t + 1);
            return;
        }
    }
}

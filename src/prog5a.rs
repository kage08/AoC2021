use std::collections::HashMap;

use crate::utils::parsefile;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(a: i32, b: i32) -> Point {
        Point { x: a, y: b }
    }
}

fn get_all_points(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut ans: Vec<Point> = Vec::new();
    if p1.x == p2.x {
        let x = p1.x;
        if p1.y > p2.y {
            for y in p2.y..(p1.y + 1) {
                ans.push(Point::new(x, y));
            }
        } else {
            for y in p1.y..(p2.y + 1) {
                ans.push(Point::new(x, y));
            }
        }
    } else if p1.y == p2.y {
        let y = p1.y;
        if p1.x > p2.x {
            for x in p2.x..(p1.x + 1) {
                ans.push(Point::new(x, y));
            }
        } else {
            for x in p1.x..(p2.x + 1) {
                ans.push(Point::new(x, y));
            }
        }
    } else if p1.x - p2.x == p1.y - p2.y {
        if p1.x > p2.x {
            for x in p2.x..(p1.x + 1) {
                ans.push(Point::new(x, p2.y + (x - p2.x)));
            }
        } else {
            for x in p1.x..(p2.x + 1) {
                ans.push(Point::new(x, p1.y + (x - p1.x)));
            }
        }
    } else if p1.x - p2.x == p2.y - p1.y {
        if p1.x > p2.x {
            for x in p2.x..(p1.x + 1) {
                ans.push(Point::new(x, p2.y - (x - p2.x)));
            }
        } else {
            for x in p1.x..(p2.x + 1) {
                ans.push(Point::new(x, p1.y - (x - p1.x)));
            }
        }
    }

    return ans;
}

fn parseline(s: &String) -> Vec<Point> {
    let substrings: Vec<&str> = s.split(" -> ").collect();
    fn str_to_pt(st: &str) -> Point {
        let xy: Vec<i32> = st
            .split(",")
            .map(|nu| nu.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Point::new(xy[0], xy[1])
    }
    substrings.iter().map(|s| str_to_pt(*s)).collect()
}

fn points_to_str(p: &Point) -> String {
    let px = p.x.to_string();
    let py = p.y.to_string();
    vec![px, py].join(",")
}

pub fn main1() {
    let lines = parsefile("inputs/input5a.txt");
    let mut mp: HashMap<String, i32> = HashMap::new();

    for line in lines.iter() {
        let pvec = parseline(line);
        let points = get_all_points(&pvec[0], &pvec[1]);
        for p in points {
            let pstr = points_to_str(&p);
            if mp.contains_key(&pstr) {
                *mp.get_mut(&pstr).unwrap() += 1;
            } else {
                mp.insert(pstr, 1);
            }
        }
    }

    let mut ans = 0;
    for k in mp.keys() {
        if mp[k] > 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

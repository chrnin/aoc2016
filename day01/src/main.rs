use num::complex::Complex;
use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(filename: &str) -> i32 {
    let input = read_input(filename);
    let turn_left = Complex::new(0, 1);
    let mut dir = Complex::new(1, 0);
    let mut pos = Complex::new(0, 0);
    input.iter().for_each(|(turn, walk)| {
        if turn == &'R' {
            dir /= turn_left;
        } else {
            dir *= turn_left;
        }
        pos += walk * dir;
    });
    pos.re.abs() + pos.im.abs()
}

fn second(filename: &str) -> i32 {
    let input = read_input(filename);
    let mut history = Vec::new();
    let turn_left = Complex::new(0, 1);
    let mut dir = Complex::new(1, 0);
    let mut pos = Complex::new(0, 0);
    for (turn, walk) in input {
        if turn == 'R' {
            dir /= turn_left;
        } else {
            dir *= turn_left;
        }
        for _ in 0..walk {
            pos += dir;
            if !history.contains(&pos) {
                history.push(pos);
            } else {
                return  (pos.re as i32).abs() + (pos.im as i32).abs()
            }
        }
    }
    return 0
}

fn read_input(filename: &str) -> Vec<(char, i32)> {
    let content = fs::read_to_string(filename).expect("can't read");
    content
        .lines()
        .nth(0)
        .unwrap()
        .split(", ")
        .map(|dir| (dir.chars().nth(0).unwrap(), dir[1..].parse().unwrap()))
        .collect()
}

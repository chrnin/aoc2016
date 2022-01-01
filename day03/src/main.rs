use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(filename: &str) -> usize {
    let input = read_input(filename);
    input
        .iter()
        .filter(|&t| {
            let mut t2 = t.clone();
            t2.sort();
            return t2[0] + t2[1] > t2[2];
        })
        .count()
}

fn second(filename: &str) -> usize {
    let input = read_input(filename);
    let mut count: usize = 0;
    for i in 0..input.len() / 3 {
        let mut t0 = vec![input[i * 3][0], input[i * 3 + 1][0], input[i * 3 + 2][0]];
        let mut t1 = vec![input[i * 3][1], input[i * 3 + 1][1], input[i * 3 + 2][1]];
        let mut t2 = vec![input[i * 3][2], input[i * 3 + 1][2], input[i * 3 + 2][2]];
        t0.sort();
        t1.sort();
        t2.sort();
        if t0[0] + t0[1] > t0[2] {
            count += 1
        }
        if t1[0] + t1[1] > t1[2] {
            count += 1
        }
        if t2[0] + t2[1] > t2[2] {
            count += 1
        }
    }
    count
}
fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let content = fs::read_to_string(filename).expect("can't read");
    content
        .lines()
        .map(|line| {
            let l1 = line[2..5].trim().parse().unwrap();
            let l2 = line[7..10].trim().parse().unwrap();
            let l3 = line[12..15].trim().parse().unwrap();
            return vec![l1, l2, l3];
        })
        .collect()
}

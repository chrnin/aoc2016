use std::collections::HashMap;
use std::fs;

fn main() {
    let (first, second) = day06("input");
    println!("first: {}", first);
    println!("second: {}", second);
}

fn day06(filename: &str) -> (String, String) {
    let input = read_input(filename);
    let mut stats = Vec::new();

    for _ in 0..input[0].len() {
        stats.push(HashMap::new());
    }
    for line in read_input(filename) {
        for (i, c) in line.chars().enumerate() {
            *stats[i].entry(c).or_insert(0) += 1
        }
    }

    let mut password: String = "".to_string();
    for stat in &stats {
        let mut chars: Vec<_> = stat.keys().collect();
        chars.sort_by(|&a, &b| stat[b].cmp(&stat[a]));
        password.push(*chars[0]);
    }
    let mut password2: String = "".to_string();
    for stat in &stats {
        let mut chars: Vec<_> = stat.keys().collect();
        chars.sort_by(|&a, &b| stat[a].cmp(&stat[b]));
        password2.push(*chars[0]);
    }
    (password, password2)
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect()
}

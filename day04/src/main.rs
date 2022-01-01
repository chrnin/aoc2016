use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(filename: &str) -> u32 {
    let input = read_input(filename);
    input
        .iter()
        .filter(|(name, _, checksum)| check(name, checksum))
        .map(|(_, id, _)| id)
        .sum()
}

fn second(filename: &str) -> u32 {
    let input = read_input(filename);
    let ciphers: Vec<_> = input
        .iter()
        .filter(|(name, _, checksum)| check(name, checksum))
        .collect();
    *ciphers
        .iter()
        .map(|(name, id, _)| {
            (
                name.chars().map(|c| translate(&c, id)).collect::<String>(),
                id,
            )
        })
        .filter(|(name, _)| name == "northpole object storage")
        .nth(0)
        .unwrap()
        .1
}
fn check(name: &String, checksum: &String) -> bool {
    let mut letters = HashMap::new();
    for c in name.chars() {
        if c != '-' {
            *letters.entry(c).or_insert(0) += 1;
        }
    }
    let mut check: Vec<&char> = letters.keys().collect();
    check.sort_by(|&a, &b| order(a, b, &letters));
    let true_checksum: &String = &check[..5].iter().map(|&c| c).collect();
    true_checksum == checksum
}

fn translate(c: &char, id: &u32) -> char {
    if c == &'-' {
        return ' ';
    } else {
        ((((*c as u8 as u32 - 97) + id) % 26) + 97) as u8 as char
    }
}

fn order(a: &char, b: &char, letters: &HashMap<char, i32>) -> Ordering {
    if letters[a] > letters[b] {
        return Ordering::Less;
    } else if letters[a] == letters[b] {
        if a < b {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    } else {
        return Ordering::Greater;
    }
}

fn read_input(filename: &str) -> Vec<(String, u32, String)> {
    let content = fs::read_to_string(filename).expect("can't read");
    let mut ret = Vec::new();
    for line in content.lines() {
        let pos = line.rfind('-').unwrap();
        let sec = &line[pos + 1..];
        let mut id_checksum = sec.split('[');
        let id: u32 = id_checksum.next().unwrap().parse().unwrap();
        let checksum = id_checksum.next().unwrap();
        ret.push((
            line[..pos].to_string(),
            id,
            checksum[..checksum.len() - 1].to_string(),
        ));
    }
    ret
}

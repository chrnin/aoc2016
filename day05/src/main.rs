use md5;
use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(filename: &str) -> String {
    let input = read_input(filename);
    let mut password: String = "".to_string();
    let mut from = 0;
    for _ in 0..8 {
        let (f, c) = seek_md5(&input, from);
        password.push(c);
        from = f + 1;
    }
    password
}

fn second(filename: &str) -> String {
    let input = read_input(filename);
    let mut password: Vec<char> = vec!['_'; 8];
    let mut from = 0;
    loop {
        let (f, c, p) = seek_md5_v2(&input, from);
        if password[p as usize] == '_' {
            password[p as usize] = c;
            if !password.contains(&'_') {
                return password.iter().collect();
            }
        }
        from = f + 1;
    }
}
fn seek_md5(input: &String, from: i64) -> (i64, char) {
    let mut i = from;
    loop {
        let s = format!("{}{}", input, i);
        let digest = format!("{:x}", md5::compute(s.clone()));
        if &digest[0..5] == "00000" {
            return (i, digest.chars().nth(5).unwrap());
        }
        i += 1;
    }
}

fn seek_md5_v2(input: &String, from: i64) -> (i64, char, i64) {
    let mut i = from;
    loop {
        let s = format!("{}{}", input, i);
        let digest = format!("{:x}", md5::compute(s.clone()));
        if &digest[0..5] == "00000" {
            if let Some(pos) = digest.chars().nth(5).unwrap().to_digit(10) {
                if pos < 8 {
                    return (i, digest.chars().nth(6).unwrap(), pos as i64);
                }
            }
        }
        i += 1;
    }
}

fn read_input(filename: &str) -> String {
    let content = fs::read_to_string(filename).expect("lecture du fichier impossible");
    return content[..content.len() - 1].to_string();
}

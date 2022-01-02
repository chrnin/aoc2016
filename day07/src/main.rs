use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(filename: &str) -> usize {
    let input = read_input(filename);
    input.iter().filter(|&line| line_is_abba(line)).count()
}

fn line_is_abba(line: &String) -> bool {
    let (outside, inside) = parse(line);
    outside.iter().any(|line| is_abba(&line)) && inside.iter().all(|line| !is_abba(line))
}

fn is_abba(word: &str) -> bool {
    for (i, _) in word[..word.len() - 3].chars().enumerate() {
        if word.chars().nth(i).unwrap() == word.chars().nth(i + 3).unwrap()
            && word.chars().nth(i + 1).unwrap() == word.chars().nth(i + 2).unwrap()
            && word.chars().nth(i).unwrap() != word.chars().nth(i + 1).unwrap()
        {
            return true;
        }
    }
    false
}

fn second(filename: &str) -> usize {
    let input = read_input(filename);
    input.iter().filter(|&line| line_is_aba(line)).count()
}

fn line_is_aba(line: &String) -> bool {
    let (outside, inside) = parse(line);
    let aba = outside.iter().flat_map(|word| lookup_aba(word));
    let mut bab = aba.map(|aba| aba_to_bab(aba));
    bab.any(|bab| inside.iter().any(|word| word.contains(&bab)))
}

fn aba_to_bab(aba: String) -> String {
    format!(
        "{}{}{}",
        aba.chars().nth(1).unwrap(),
        aba.chars().nth(0).unwrap(),
        aba.chars().nth(1).unwrap()
    )
}
fn lookup_aba(word: &String) -> Vec<String> {
    let mut aba = Vec::new();
    for (i, _) in word[..word.len() - 2].chars().enumerate() {
        if word.chars().nth(i).unwrap() == word.chars().nth(i + 2).unwrap()
            && word.chars().nth(i).unwrap() != word.chars().nth(i + 1).unwrap()
        {
            aba.push(format!(
                "{}{}{}",
                word.chars().nth(i).unwrap(),
                word.chars().nth(i + 1).unwrap(),
                word.chars().nth(i).unwrap()
            ))
        }
    }
    aba
}
fn parse(line: &String) -> (Vec<String>, Vec<String>) {
    let closers: Vec<_> = line.match_indices(']').collect();
    let mut start: usize = 0;
    let mut outside = Vec::new();
    let mut inside = Vec::new();
    for (id, (open, _)) in line.match_indices('[').enumerate() {
        outside.push(line[start..open].to_string());
        inside.push(line[open + 1..closers[id].0].to_string());
        start = closers[id].0 + 1;
    }
    outside.push(line[start..].to_string());
    (outside, inside)
}

fn read_input(filename: &str) -> Vec<String> {
    let content = fs::read_to_string(filename).expect("can't read");
    return content.lines().map(|s| s.to_string()).collect();
}

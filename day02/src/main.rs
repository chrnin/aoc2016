use std::fs;

fn main() {
    println!("first: {}", first("input"));
    println!("second: {}", second("input"));
}

fn first(filename: &str) -> i32 {
    let input = read_input(filename);
    let mut numbers = Vec::new();
    for operations in input {
        let mut number = [1, 1];
        for op in operations {
            if op == 'D' {
                number[1] = 2.min(number[1] + 1);
            } else if op == 'U' {
                number[1] = 0.max(number[1] - 1);
            } else if op == 'R' {
                number[0] = 2.min(number[0] + 1);
            } else if op == 'L' {
                number[0] = 0.max(number[0] - 1);
            }
        }
        numbers.push(number[1] * 3 + number[0] + 1)
    }
    numbers
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &n)| n * 10_i32.pow(i as u32))
        .sum()
}

fn second(filename: &str) -> String {
    let input = read_input(filename);
    let mut numbers = Vec::new();
    let digit = [[' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' ']];

    for operations in input {
        let mut number = [-2, 0];
        for op in operations {
            let new_number;
            if op == 'D' {
                new_number = [number[0], number[1] - 1];
            } else if op == 'U' {
                new_number = [number[0], number[1] + 1];
            } else if op == 'L' {
                new_number = [number[0] - 1, number[1]];
            } else {
                // 'R'
                new_number = [number[0] + 1, number[1]]
            }
            if (new_number[0] as i32).abs() + (new_number[1] as i32).abs() <= 2 {
                number = new_number;
            }
        }
        numbers.push(number)
    }
    numbers.iter().map(|[x,y]| digit[(-y+2) as usize][(x + 2) as usize]).collect()
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(filename).expect("can't read");
    content.lines().map(|line| line.chars().collect()).collect()
}

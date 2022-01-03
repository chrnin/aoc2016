use std::fs;

fn main() {
    let input = read_input("input");
    let mut screen = screen(50, 6);
    for operation in &input {
        apply(&mut screen, operation);
    }
    println!("first: {}", screen.iter().flat_map(|p| p).filter(|&&p| p).count());
    println!("second: ");
    print(&screen);
}



fn apply(screen: &mut Vec<Vec<bool>>, operation: &Operation) {
    match operation {
        &Operation::Rect(x,y) => {rect(screen, x, y)},
        &Operation::RotateRow(x,y) => {rotaterow(screen, x, y) },
        &Operation::RotateColumn(x, y) => {rotatecol(screen, x,y) }
    }
}

fn print(screen: &Vec<Vec<bool>>) {
    screen.iter().for_each(|line| {line.iter().for_each(|&pixel| if pixel {print!("▮")} else {print!(" ")}); print!("\n") });
}
fn rect(screen: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    for i in 0..y {
        for j in 0..x {
            screen[i][j] = true;
        }
    }
}

fn rotaterow(screen: &mut Vec<Vec<bool>>, y: usize, x: usize) {
    let x = screen[0].len() - x%screen[0].len();
    let y = y%screen.len();
    let row = screen[y].clone();
    let mut new_row = row[x..].to_vec();
    new_row.append(&mut row[..x].to_vec());
    screen[y] = new_row;
}

fn rotatecol(screen: &mut Vec<Vec<bool>>, x: usize, y: usize) {
    let x = x%screen[0].len();
    let y = screen.len() - y%screen.len();
    let mut col = Vec::new();
    for i in 0..screen.len() {
        col.push(screen[i][x]);
    }
    let mut new_col = col[y..].to_vec();
    new_col.append(&mut col[..y].to_vec());
    for i in 0..screen.len() {
        screen[i][x] = new_col[i];
    }
}

fn screen(x: usize, y: usize) -> Vec<Vec<bool>> {
    let mut screen = Vec::new();
    for _ in 0..y {
        let mut line = Vec::new();
        for _ in 0..x {
            line.push(false);
        }
        screen.push(line);
    }
    screen
}


#[derive(Debug)]
enum Operation {
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
    Rect(usize, usize),
}
fn read_input(filename: &str) -> Vec<Operation> {
    let content = fs::read_to_string(filename).expect("can't read");
    content.lines().map(|line| parse_input_line(line)).collect()
}

fn parse_input_line(line: &str) -> Operation {
    if &line[0..2] == "re" {
        let mut dimstr = line[5..].split("x");
        let x = dimstr.next().unwrap().parse().unwrap();
        let y = dimstr.next().unwrap().parse().unwrap();
        return Operation::Rect(x,y)
    } else {
        if &line[7..9] == "ro" {
            let mut dimstr = line[13..].split(" by ");
            let x = dimstr.next().unwrap().parse().unwrap();
            let y = dimstr.next().unwrap().parse().unwrap();
            return Operation::RotateRow(x,y)
        } else {
            let mut dimstr = line[16..].split(" by ");
            let x = dimstr.next().unwrap().parse().unwrap();
            let y = dimstr.next().unwrap().parse().unwrap();
            return Operation::RotateColumn(x,y)
        }
    }
}

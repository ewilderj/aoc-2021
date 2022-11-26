use std::collections::HashMap;

fn parse_coords(s: &str) -> (i32, i32) {
    let (xs, ys) = s.split_once(",").unwrap();
    return (xs.parse::<i32>().unwrap(), ys.parse::<i32>().unwrap());
}

fn find_dir(a: i32, b:i32) -> i32 {
    if a < b { 1 } else if a > b { -1 } else { 0 }
}

fn draw(b: &mut HashMap<(i32, i32), i32>, f: &str, t: &str, diag: bool) {
    let (x0, y0) = parse_coords(f);
    let (x1, y1) = parse_coords(t);

    if x0 != x1 && y0 != y1 && !diag {
        // diagonal, so skip
    } else {
        let (dx, dy) = (find_dir(x0, x1), find_dir(y0, y1));
        let (mut x, mut y) = (x0, y0);
        // println!("from {} to {} dx {} dy {}", f, t, dx, dy);
        while (x != x1 || y != y1)  {
            b.entry((x, y)).and_modify(|v| { *v += 1}).or_insert(1);
            x = x + dx;
            y = y + dy;
        }
        b.entry((x, y)).and_modify(|v| { *v += 1}).or_insert(1);
    }
}

fn dump_board(b: &HashMap<(i32, i32), i32>, w: i32, h: i32) {
    for y in 0..=h {
        for x in 0..=w {
            if let Some(m) = b.get(&(x,y)) {
                print!("{}", m);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn main() {
    let l = include_str!("../input.txt").lines();

    let mut board = HashMap::<(i32, i32), i32>::new();

    for s in l.clone() {
          let (from, to) = s.split_once(" -> ").unwrap();
          draw(&mut board, &from, &to, false);
    }
    let part1 = board.values().filter(|v| **v > 1).count();

    println!("part1: {}", part1);

    let mut board = HashMap::<(i32, i32), i32>::new();
    for s in l.clone() {
          let (from, to) = s.split_once(" -> ").unwrap();
          draw(&mut board, &from, &to, true);
    }
    let part2 = board.values().filter(|v| **v > 1).count();
    // dump_board(&board, 9, 9);
    println!("part2: {}", part2);


}

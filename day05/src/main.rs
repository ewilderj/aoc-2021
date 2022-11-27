use std::collections::HashMap;

type Board = HashMap<(i32, i32), u32>;

fn parse_coords(s: &str) -> (i32, i32) {
    let (xs, ys) = s.split_once(",").unwrap();
    return (xs.parse::<i32>().unwrap(), ys.parse::<i32>().unwrap());
}

fn draw(b: &mut Board, (x0, y0): (i32, i32), (x1, y1): (i32, i32), no_d: bool) {
    if no_d && x0 != x1 && y0 != y1 {
        return;
    } else {
        let (dx, dy) = ((x1-x0).signum(), (y1-y0).signum());
        let (mut x, mut y) = (x0, y0);

        while x != (x1 + dx) || y != (y1 + dy) {
            b.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
            x += dx;
            y += dy;
        }
    }
}

fn main() {
    let (mut board1, mut board2) = (Board::new(), Board::new());

    for s in include_str!("../input.txt").lines() {
        let (a, b) = s.split_once(" -> ").unwrap();
        let (f, t) = (parse_coords(a), parse_coords(b));
        draw(&mut board1, f, t, true);
        draw(&mut board2, f, t, false);
    }
    let part1 = board1.values().filter(|v| **v > 1).count();
    let part2 = board2.values().filter(|v| **v > 1).count();

    println!("part1: {}\npart2: {}", part1, part2);
}

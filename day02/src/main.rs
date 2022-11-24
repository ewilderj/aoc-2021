
fn delta_from_instruction(i: &str) -> (i32, i32) {
    let v: Vec<&str> = i.split(" ").collect();
    let n: i32 = v[1].parse::<i32>().unwrap();

    return match v[0] {
        "forward" => (n, 0),
        "down" => (0, n),
        "up" => (0, -n),
        _ => todo!()
    }
}

fn main() {
    let l = include_str!("../input.txt")
        .lines()
        .map(|s| delta_from_instruction(s));

    let p1 = l.clone()
        .fold((0, 0), |a, (dx, dy)| (a.0 + dx, a.1 + dy));

    println!("part1: {}", p1.0 * p1.1);

    // acc is 3-tuple (aim, x, y)
    let p2 = l.clone()
        .fold((0, 0, 0), |a, (dx, dy)| (a.0 + dy, a.1 + dx, a.2 + (a.0 * dx)));

    println!("part2: {}", p2.1 * p2.2);

}

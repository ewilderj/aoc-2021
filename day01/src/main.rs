use itertools::Itertools;

fn count_increases(a: impl IntoIterator<Item = u32>) -> usize {
    return a
        .into_iter()
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| a < b)
        .count();
}

fn main() {
    let i = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<u32>().unwrap());

    let r = count_increases(i.clone());

    println!("part1: {}", r);

    let r2 = count_increases(
        i.clone()
            .tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| a + b + c),
    );

    println!("part2: {}", r2);
}

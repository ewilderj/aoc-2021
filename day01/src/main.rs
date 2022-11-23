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

    //  more efficient version of part2, as 2 items are in common
    //  over sliding 3-tuples and don't need to be considered
    let r3 = i
        .clone()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(a, _, _, d)| a < d)
        .count();

    println!("part2': {}", r3);
}

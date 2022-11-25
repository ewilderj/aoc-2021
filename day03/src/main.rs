use std::collections::HashSet;

fn main() {
    let mut l = include_str!("../test.txt").lines().peekable();
    let w: usize = l.peek().unwrap().len();

    let m: Vec<u32> = l
        .map(|s| s.chars().map(|d| d.to_digit(2).unwrap()))
        .flatten()
        .collect();

    let n = m.len();
    let h = n / w;
    // println!("w {} h {}, n {}", w, h, n);

    let mut t_m = vec![0; n];
    transpose::transpose(&m, &mut t_m, w, h);

    let threshold = h / 2;
    let t_c: Vec<char> = t_m
        .chunks_exact(h)
        .map(|c| c.iter().filter(|&z| *z == 1))
        .map(|c| if c.count() > threshold { '1' } else { '0' })
        .collect();

    let gamma = u32::from_str_radix(&String::from_iter(t_c), 2).unwrap();
    let epsilon = gamma ^ (1 << w) - 1;

    println!("part1: {}", gamma * epsilon);

    // part2: we go back to our chunked_array
    let mut bois = HashSet::new();
    for i in m.chunks_exact(w) {
        bois.insert(i);
    }

    println!("{:?}", bois);

    let mut bit = 0;
    while bit < w && bois.len() > 1 {
        let mut ones = 0;
        let blen = bois.len();
        for b in bois.iter() {
            if b[bit] == 1 {
                ones = ones + 1
            }
        }

        let target = if (2 * ones >= blen) { 0 } else { 1 };
        println!("bit {} count {} blen {} target {}", bit, ones, blen, target);

        for v in bois.clone().iter() {
            if v[bit] == target {
                bois.remove(v);
            }
        }
        println!("{:?}", bois);
        bit = bit + 1
    }
    let n: Vec<char> = bois
        .iter()
        .next()
        .unwrap()
        .iter()
        .map(|d| if *d == 1 { '1' } else { '0' })
        .collect();

    println!("{:?}", u32::from_str_radix(&String::from_iter(n), 2).unwrap());
}

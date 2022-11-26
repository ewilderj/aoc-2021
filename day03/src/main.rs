use std::collections::HashSet;

fn main() {
    let mut l = include_str!("../input.txt").lines().peekable();
    let w: usize = l.peek().unwrap().len();

    let m: Vec<char> = l
        .map(|s| s.chars())
        .flatten()
        .collect();

    let n = m.len();
    let h = n / w;

    // now w and h are width and height of our array of binary
    // and to work on the columns, we need to transpose first
    let mut t_m = vec!['0'; n];
    transpose::transpose(&m, &mut t_m, w, h);

    // for each column (transposed row of h items) count 1s, and
    // if they're in the majority, report 1 else 0
    let t_c: Vec<char> = t_m
        .chunks_exact(h)
        .map(|c| c.iter().filter(|&z| *z == '1'))
        .map(|c| if 2 * c.count() > h { '1' } else { '0' })
        .collect();

    // utility to convert a vector of ascii chars representing binary
    let char_vec_to_num = |v: Vec<char>| -> u32 { u32::from_str_radix(&String::from_iter(v), 2).unwrap() };

    let gamma = char_vec_to_num(t_c);
    let epsilon = gamma ^ (1 << w) - 1;

    println!("part1: {}", gamma * epsilon);

    // part2:
    // our comparator checks for the o2 and co2 cases appropriately. c is count of 1s,
    // l is length of the set being considered still
    let c_f = |c: usize, l: usize, o2: bool| -> bool {
        if o2 {
            2 * c >= l
        } else {
            2 * c < l
        }
    };

    let count_em = |is_o2: bool| -> u32 {
        // init candidate set bois from rows
        let mut bois: HashSet<&[char]> = HashSet::from_iter(m.chunks_exact(w));
        let mut bit = 0;
        while bit < w && bois.len() > 1 {
            let ct = bois.iter().filter(|b| b[bit] == '1').count();
            let keep = if c_f(ct, bois.len(), is_o2) { '1' } else { '0' };
            bois.retain(|&v| v[bit] == keep);
            bit += 1;
        }
        return char_vec_to_num(bois.iter().next().unwrap().to_vec());
    };

    println!("part2: {}", count_em(true) * count_em(false));
}

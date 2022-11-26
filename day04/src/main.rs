use std::collections::HashSet;

fn main() {
    let mut l = include_str!("../input.txt").lines();

    let calls: Vec<u32> = l
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    // we're going to store boards as vectors of 25 <u32>s
    let mut boards: Vec<Vec<u32>> = Vec::new();
    loop {
        match l.next() {
            // blank line is the introducer for 5 more lines of board
            Some("") => {
                let b: Vec<_> = (0..5)
                    .into_iter()
                    .map(|_| {
                        l.next()
                            .unwrap()
                            .split_whitespace()
                            .map(|n| n.parse::<u32>().unwrap())
                    })
                    .flatten()
                    .collect();
                boards.push(b);
            }
            _ => break,
        }
    }

    let num_boards = boards.len();

    // some helper functions

    // find the index of a call in a board
    let find_num = |n: usize, t: u32| -> Option<usize> { boards[n].iter().position(|&x| x == t) };

    // indices of winning combination indices
    let winning_combos: Vec<HashSet<usize>> = (0..5)
        .into_iter()
        .map(|n| {
            vec![
                HashSet::<usize>::from([n, n + 5, n + 10, n + 15, n + 20]),
                HashSet::<usize>::from([n * 5, n * 5 + 1, n * 5 + 2, n * 5 + 3, n * 5 + 4]),
            ]
        })
        .flatten()
        .collect();

    // look for a winning combination given a set of indices
    let check_board = |m: &HashSet<usize>| -> bool {
        for i in winning_combos.iter() {
            if m.is_superset(&i) {
                return true;
            }
        }
        return false;
    };

    // now to play the game; first time we break with the first
    // board. second time, we let all the boards score
    for part in [1, 2] {
        // initialize an empty set of indices per board
        let mut marks: Vec<HashSet<usize>> = Vec::new();
        for _ in 0..num_boards {
            marks.push(HashSet::new());
        }
        // keep track of winning board
        let mut winners: Vec<usize> = Vec::new();
        let mut score = 0;
        'cloop: for call in calls.clone() {
            for i in 0..num_boards {
                // play only unwon boards
                if !winners.contains(&i) {
                    // if we have the number, record the index
                    if let Some(pos) = find_num(i, call) {
                        marks[i].insert(pos);
                    }
                    // if we've won, then calculate score
                    if check_board(&marks[i]) {
                        score = call
                            * (0..25)
                                .into_iter()
                                .filter(|n| !&marks[i].contains(n))
                                .fold(0, |a, n| a + boards[i][n]);
                        winners.push(i);

                        if part == 1 {
                            break 'cloop;
                        }
                    }
                }
            }
        }
        println!("part{}: {}", part, score);
    }
}

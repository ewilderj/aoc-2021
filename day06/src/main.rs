fn main() {
    let mut fish: Vec<u64> = include_str!("../input.txt")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // make an untouched copy for part 2
    let mut fish2 = Vec::from(fish.clone());

    // brute force simulation for part1, but we'll need to find
    // something smart for part2
    for d in 0..80 {
        for i in 0..fish.len() {
            if (fish[i] == 0) {
                fish[i] = 6;
                fish.push(8);
            } else {
                fish[i] = fish[i] - 1;
            }
        }
    }
    println!("part1: {}", fish.len());

    // let's move our focus to how many fish are spawned each day
    // we can then sum that to find how many fish exist in total
    const DAYS: usize = 256;

    let mut fishies: [u64; DAYS + 9] =  [0; DAYS + 9];

    // for the initial fish, record their projected spawn
    for f in fish2.iter () {
        fishies[*f as usize] += 1;
    }

    for d in 0..DAYS {
        // fishies[d] fish are born today! hooray!
        //
        // everyone alive today will spawn in 7 days time
        fishies[d + 7] += fishies[d];
        // every newly-spawned fish today will spawn in 9 days time
        fishies[d + 9] += fishies[d];
    }

    // answer is newly alive fish, plus the initial population
    let t = fish2.len() as u64 + &fishies[0..DAYS].iter().sum::<u64>();
    println!("part2: {}", t);

    // check out https://github.com/timvisee/advent-of-code-2021/blob/master/day06b/src/
    // for a solution that realizes this can all be done with modulo 9 arithmetic
    // disclaimer: I only solved this efficiently after looking at Tim's ideas.
    // Life is short, and you have to keep this fun!
}

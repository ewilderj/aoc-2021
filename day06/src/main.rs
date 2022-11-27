
fn main() {
    let mut fish: Vec<u32> = include_str!("../input.txt")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut fish2 = VecDeque::from(fish.clone());

    // brute force for part1, but we'll need to find
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

    const TOTAL_DAYS: u32 = 256;
    const PERIOD: u32 = 7;

    fn will_spawn_on(days_before_spawn: u32) -> VecDeque<u32> {
        if (days_before_spawn >= TOTAL_DAYS) {
            return VecDeque::new();
        } else {
            let n = (TOTAL_DAYS - days_before_spawn - 1) / PERIOD + 1;
            return (0..n).map(|i| 9 + days_before_spawn + i * PERIOD).collect();
        }
    }

    let mut t = fish2.len();
    println!("starting with {} fish", t);
    loop {
        if fish2.len() == 0 {
            println!("t {}", t);
            break;
        }
        let i = fish2.pop_front().unwrap();
        let mut d: VecDeque<u32> = will_spawn_on(i);
        let n = d.len();
        t = t + n;
        // println!("++ {:?}", d);
        if n > 0 { fish2.append(&mut d); }
        // println!("({}) {} -> n {} {:?}", t, i, n, fish2);

    }
    // assume 1 fish, on day 0 initially
    //
    // day1: (daysT - daysI) / 7 = number of 1st gen spawns
    // daysP = daysT-daysI
    // first spawner will have: (daysP - 9) / 7 = 2nd gen spawns
}

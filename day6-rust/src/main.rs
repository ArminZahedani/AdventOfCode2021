use std::collections::HashMap;

fn main() {
    println!("Silver star: {}", solve(80));
    println!("Gold star: {}", solve(256));
}

fn solve(time_steps: usize) -> usize {
    let input = include_str!("input.txt");

    let mut fish: HashMap<u32, usize> = HashMap::new();

    let fishes: Vec<u32> = input
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse().unwrap())
        .collect();

    for element in fishes.iter() {
        let update = fish.entry(*element).or_insert(0);
        *update += 1;
    }

    for _i in 0..time_steps {
        let mut new_fish_map: HashMap<u32, usize> = HashMap::new();
        let new_fish = fish.entry(0).or_insert(0).clone();
        for index in 0..9 {
            if index != 0 {
                new_fish_map.insert(index - 1, *fish.entry(index).or_insert(0));
                continue;
            }
        }

        let existing = new_fish_map.entry(6).or_insert(0).clone();
        new_fish_map.insert(6, existing + *fish.entry(0).or_insert(0));

        new_fish_map.insert(8, new_fish);
        fish = new_fish_map;
    }
    let mut count = 0;
    for (_key, value) in fish {
        count += value;
    }
    count
}

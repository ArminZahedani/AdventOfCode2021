use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn task1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut counts: HashMap<usize, Vec<i32>> = HashMap::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (i, element) in line.chars().enumerate() {
            let vector = counts.entry(i).or_insert(vec![0, 0]);
            let binary: usize = match element {
                '0' => 0,
                '1' => 1,
                _ => panic!("Found non-binary digit"),
            };
            vector[binary] = vector[binary] + 1;
        }
    }
    let mut gamma: Vec<i32> = vec![0; counts.len()];
    let mut beta: Vec<i32> = vec![0; counts.len()];

    for (key, element) in &counts {
        if element[0] > element[1] {
            gamma[*key] = 0;
            beta[*key] = 1;
        } else {
            gamma[*key] = 1;
            beta[*key] = 0;
        }
    }
    let a: String = gamma.iter().map(ToString::to_string).collect();
    let a: u32 = u32::from_str_radix(&a, 2).unwrap();

    let b: String = beta.iter().map(ToString::to_string).collect();
    let b: u32 = u32::from_str_radix(&b, 2).unwrap();

    println!("Silver star: {}", a * b);
}

fn task2() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut oxygen: Vec<String> = Vec::new();
    let mut co2: Vec<String> = Vec::new();
    let mut counts_co2 = vec![0, 0];
    let mut counts_oxygen = vec![0, 0];

    let till: usize = reader.by_ref().lines().nth(0).unwrap().unwrap().len();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        oxygen.push(line.clone());
        co2.push(line.clone());
    }

    for line in oxygen.clone() {
        let element = line.chars().nth(0).unwrap();
        let binary: usize = match element {
            '0' => 0,
            '1' => 1,
            _ => panic!("Found non-binary digit"),
        };
        counts_oxygen[binary] = counts_oxygen[binary] + 1;
    }
    for line in co2.clone() {
        let element = line.chars().nth(0).unwrap();
        let binary: usize = match element {
            '0' => 0,
            '1' => 1,
            _ => panic!("Found non-binary digit"),
        };
        counts_co2[binary] = counts_co2[binary] + 1;
    }
    for key in 0..till {
        if oxygen.len() > 1 {
            let matcher = match counts_oxygen[0].cmp(&counts_oxygen[1]) {
                Ordering::Less => '1',
                Ordering::Greater => '0',
                Ordering::Equal => '1',
            };
            oxygen = oxygen
                .iter()
                .filter(|string| string.chars().nth(key).unwrap() == matcher)
                .cloned()
                .collect();
        }
        if co2.len() > 1 {
            let matcher = match counts_co2[0].cmp(&counts_co2[1]) {
                Ordering::Less => '0',
                Ordering::Greater => '1',
                Ordering::Equal => '0',
            };
            co2 = co2
                .iter()
                .filter(|string| string.chars().nth(key).unwrap() == matcher)
                .cloned()
                .collect();
        }
        if key != till - 1 {
            counts_oxygen = vec![0, 0];
            for line in oxygen.clone() {
                let element = line.chars().nth(key + 1).unwrap();
                let binary: usize = match element {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Found non-binary digit"),
                };
                counts_oxygen[binary] = counts_oxygen[binary] + 1;
            }
            counts_co2 = vec![0, 0];
            for line in co2.clone() {
                let element = line.chars().nth(key + 1).unwrap();
                let binary: usize = match element {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("Found non-binary digit"),
                };
                counts_co2[binary] = counts_co2[binary] + 1;
            }
        }
    }
    println!(
        "Gold star: {:?}",
        u32::from_str_radix(&oxygen[0], 2).unwrap() * u32::from_str_radix(&co2[0], 2).unwrap()
    );
}

fn main() {
    task1();
    task2();
}

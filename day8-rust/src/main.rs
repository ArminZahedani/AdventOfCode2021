use itertools::izip;
use std::collections::HashMap;

fn silver() {
    let input = include_str!("input.txt");

    let mut vec: Vec<Vec<String>> = vec![];
    for element in input.lines() {
        let x = element
            .split("|")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let x = &x[1];
        vec.push(
            x.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }

    let mut count = 0;
    for element in vec.iter() {
        for element2 in element {
            if element2.len() == 2
                || element2.len() == 4
                || element2.len() == 3
                || element2.len() == 7
            {
                count += 1;
            }
        }
    }
    println!("Silver star: {}", count);
}

fn gold() {
    let input = include_str!("input.txt");

    let mut vec: Vec<Vec<String>> = vec![];
    let mut patterns: Vec<Vec<String>> = vec![];
    for element in input.lines() {
        let x = element
            .split("|")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let numbers = &x[1];

        vec.push(
            numbers
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .clone(),
        );

        let observed = &x[0];
        patterns.push(
            observed
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .clone(),
        );
    }
    let mut count = 0;
    for (element, numbers) in izip!(patterns, vec) {
        let seven: String;
        let one: String;
        let eight: String;
        let four: String;

        let mut patterns_to_numbers: HashMap<String, usize> = HashMap::new();
        let mut patterns_to_numbers_sorted: HashMap<String, usize> = HashMap::new();
        let mut possible_mapping: HashMap<char, char> = HashMap::new();
        let mut possible_mapping_reversed: HashMap<char, char> = HashMap::new();

        seven = element.clone().into_iter().find(|s| s.len() == 3).unwrap();
        one = element.clone().into_iter().find(|s| s.len() == 2).unwrap();
        eight = element.clone().into_iter().find(|s| s.len() == 7).unwrap();
        four = element.clone().into_iter().find(|s| s.len() == 4).unwrap();

        for c in seven.chars() {
            if !one.contains(c) {
                possible_mapping.insert(c, 'a');
                possible_mapping_reversed.insert('a', c);
            }
        }
        let mut temp: Vec<char> = vec![];
        for c in eight.chars() {
            if !four.contains(c) && !possible_mapping.contains_key(&c) {
                temp.push(c);
            }
        }
        patterns_to_numbers.insert(eight, 8);
        patterns_to_numbers.insert(seven, 7);

        patterns_to_numbers.insert(four, 4);

        for letter in temp {
            if element
                .iter()
                .filter(|s| s.contains(letter))
                .collect::<Vec<&String>>()
                .len()
                == 4
            {
                possible_mapping.insert(letter, 'e');
                possible_mapping_reversed.insert('e', letter);
            } else if element
                .iter()
                .filter(|s| s.contains(letter))
                .collect::<Vec<&String>>()
                .len()
                == 7
            {
                possible_mapping.insert(letter, 'g');
                possible_mapping_reversed.insert('g', letter);
            }
        }
        let nine: Vec<&String> = element
            .iter()
            .filter(|s| s.len() == 6)
            .filter(|s| !s.contains(*possible_mapping_reversed.get(&'e').unwrap()))
            .collect();
        patterns_to_numbers.insert(nine[0].to_string(), 9);

        let zero: Vec<&String> = element
            .iter()
            .filter(|s| s.len() == 6)
            .filter(|&s| *s != nine[0].to_string())
            .filter(|s| s.contains(one.chars().nth(0).unwrap()))
            .filter(|s| s.contains(one.chars().nth(1).unwrap()))
            .collect();

        patterns_to_numbers.insert(zero[0].to_string(), 0);

        let six: Vec<&String> = element
            .iter()
            .filter(|s| s.len() == 6)
            .filter(|&s| *s != nine[0].to_string())
            .filter(|&s| *s != zero[0].to_string())
            .collect();
        patterns_to_numbers.insert(six[0].to_string(), 6);

        for remaining in element {
            if remaining.len() != 5 {
                continue;
            }
            if remaining.contains(*possible_mapping_reversed.get(&'e').unwrap()) {
                patterns_to_numbers.insert(remaining, 2);
            } else if remaining.contains(one.clone().chars().nth(0).unwrap())
                && remaining.contains(one.clone().chars().nth(1).unwrap())
            {
                patterns_to_numbers.insert(remaining, 3);
            } else if remaining.contains(one.clone().chars().nth(0).unwrap())
                || remaining.contains(one.clone().chars().nth(1).unwrap())
            {
                patterns_to_numbers.insert(remaining, 5);
            }
        }

        patterns_to_numbers.insert(one, 1);

        for (key, value) in &patterns_to_numbers {
            let mut new_key_ordered = key.chars().collect::<Vec<char>>();
            new_key_ordered.sort();
            patterns_to_numbers_sorted.insert(new_key_ordered.iter().collect(), *value);
        }

        let mut val = 0;
        for number in numbers {
            let mut number = number.chars().collect::<Vec<char>>();
            number.sort();
            let number: String = number.iter().collect();
            let num = patterns_to_numbers_sorted.get(&number).unwrap();
            val *= 10;
            val += num;
        }
        count += val;
    }
    println!("Gold star: {}", count);
}

fn main() {
    silver();
    gold();
}

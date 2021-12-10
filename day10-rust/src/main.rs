use std::collections::HashMap;
use std::collections::VecDeque;

fn silver() {
    let input = include_str!("input.txt");
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let opening_chars:[char; 4] = ['(', '[', '{', '<'];
    let closing_chars:[char; 4] = [')', ']', '}', '>'];
    let mut pairs: HashMap<char, char> = HashMap::new();
    pairs.insert('(', ')');
    pairs.insert('{', '}');
    pairs.insert('[', ']');
    pairs.insert('<', '>');

    let points: HashMap<char, usize> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut count = 0;

    for line in lines {
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in line.chars() {
            if opening_chars.contains(&c) {
                stack.push_front(*pairs.get(&c).unwrap());
            }
            else if closing_chars.contains(&c) {
                let a  = stack.pop_front().unwrap();
                if c != a {
                    count += points.get(&c).unwrap();
                }
            }
        }
    }
    println!("Silver star {}", count);

}

fn gold () {
    let input = include_str!("input.txt");
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let opening_chars:[char; 4] = ['(', '[', '{', '<'];
    let closing_chars:[char; 4] = [')', ']', '}', '>'];
    let mut pairs: HashMap<char, char> = HashMap::new();
    pairs.insert('(', ')');
    pairs.insert('{', '}');
    pairs.insert('[', ']');
    pairs.insert('<', '>');

    let points: HashMap<char, usize> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut scores: Vec<usize> = vec![];
    'outer: for line in lines {
        let mut count = 0;
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in line.chars() {
            if opening_chars.contains(&c) {
                stack.push_front(*pairs.get(&c).unwrap());
            }
            else if closing_chars.contains(&c) {
                let a  = stack.pop_front().unwrap();
                if c != a {
                    continue 'outer;
                }
            }
        }
        while !stack.is_empty() {
            let a = stack.pop_front().unwrap();
            count *= 5;
            count += points.get(&a).unwrap();
        }
        scores.push(count);
    }
    scores.sort();
    println!("Gold star: {}", scores.iter().nth(scores.len() / 2).unwrap());
}

fn main() {
    silver();
    gold();
}
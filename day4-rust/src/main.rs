use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn check_win(grid: &Vec<Vec<i32>>) -> bool {
    for i in 0..grid[0].len() {
        if grid[i].iter().filter(|s| **s == -1).count() == grid[0].len() {
            return true;
        }
        let mut sum: usize = 0;
        for j in 0..grid[i].len() {
            if grid[j][i] == -1 {
                sum = sum + 1;
            }
        }
        if sum == grid[i].len() {
            return true;
        }
    }
    return false;
}

fn task() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let numbers: Vec<i32> = reader
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grids: Vec<Vec<Vec<i32>>> = vec![];
    let mut board: Vec<Vec<i32>> = vec![];
    let mut solution: HashMap<usize, i32> = HashMap::new();

    let mut counter = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if index <= 1 {
            continue;
        }
        if line == "" || line == "[]" {
            grids.push(board);
            board = vec![];
            continue;
        }
        let line: Vec<i32> = line
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        board.push(line);
    }
    grids.push(board);
    let mut done: HashSet<usize> = HashSet::new();
    for number in numbers {
        for (index, board) in grids.iter_mut().enumerate() {
            if !done.contains(&index) {
                for i in 0..board[0].len() {
                    for j in 0..board[i].len() {
                        if board[i][j] == number {
                            board[i][j] = -1;
                        }
                    }
                }
                if check_win(&board) {
                    let mut sum: i32 = 0;
                    for i in 0..board[0].len() {
                        let a: i32 = board[i].iter().filter(|&s| *s != -1).sum::<i32>();
                        sum = sum + a;
                    }
                    solution.entry(counter).or_insert(number * sum);

                    counter = counter + 1;
                    done.insert(index);
                }
            }
        }
    }
    println!("Silver star: {:?}", solution.entry(0).or_insert(0));
    println!("Gold star: {:?}", solution.entry(counter - 1).or_insert(0));
}

fn main() {
    task();
}

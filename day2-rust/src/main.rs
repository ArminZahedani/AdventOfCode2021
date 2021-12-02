use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn task1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let operation = iter.next().unwrap();
        let magnitude = iter
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Not a number to parse");

        if operation == "forward" {
            horizontal = horizontal + magnitude;
        }
        if operation == "down" {
            depth = depth + magnitude;
        }
        if operation == "up" {
            depth = depth - magnitude;
        }
    }
    println!("{}", depth * horizontal)
}

fn task2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let operation = iter.next().unwrap();
        let magnitude = iter
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Not a number to parse");

        if operation == "forward" {
            horizontal = horizontal + magnitude;
            depth = depth + aim * magnitude;
        }
        if operation == "down" {
            aim = aim + magnitude;
        }
        if operation == "up" {
            aim = aim - magnitude;
        }
    }
    println!("{}", depth * horizontal)
}

fn main() {
    task1();
    task2();
}

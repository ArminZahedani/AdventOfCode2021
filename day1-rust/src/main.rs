use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn task1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut previous: i32 = 0;
    let mut count: u32 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let num: i32 = line.parse::<i32>().unwrap();
        if num > previous{
            count = count + 1;
        }
        previous = num;
    }
    count = count - 1;
    println!("Answer Silver star is: {}", count);
}

fn task2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut vec = vec![0,0,0];
    let mut count: u32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let num: i32 = line.parse::<i32>().unwrap();
        if index <= 2 {
            vec[index] = num;
            continue;
        }
        if vec.remove(0) < num {
            count = count + 1;
        }
        vec.push(num);
    }
    println!("Answer Gold star is: {}", count);
}

fn main() {
    task1();
    task2();
}

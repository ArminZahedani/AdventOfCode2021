fn silver() {
    let input = include_str!("input.txt");

    let vec: Vec<i32> = input
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let max: &i32 = vec.iter().max().unwrap();
    let mut fuels = i32::MAX;

    for i in 0..*max {
        let mut count = 0;
        for element in vec.iter() {
            count += i32::abs(element - i);
        }
        if count < fuels {
            fuels = count;
        }
    }
    println!("Silver Star {}", fuels);
}

fn gold() {
    let input = include_str!("input.txt");

    let vec: Vec<i32> = input
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let max: &i32 = vec.iter().max().unwrap();
    let mut fuels = i32::MAX;
    
    for i in 0..*max {
        let mut count = 0;
        for element in vec.iter() {
            let n = i32::abs(element - i);
            count += n * (n + 1) / 2; //formula for 1 + 2 + 3 + ... + n
        }
        if count < fuels {
            fuels = count;
        }
    }
    println!("Gold Star {}", fuels);
}
fn main() {
    silver();
    gold();
}

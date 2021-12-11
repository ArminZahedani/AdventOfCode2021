use std::collections::HashSet;
use std::collections::VecDeque;

fn silver() {
    let input = include_str!("input.txt");
    let grid: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut grid_grid: Vec<Vec<i32>> = vec![];
    let directions: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let mut count = 0;
    for element in grid {
        let mut line: Vec<i32> = vec![];
        for element2 in element.chars() {
            line.push(element2.to_digit(10).unwrap().try_into().unwrap());
        }
        grid_grid.push(line)
    }

    for _iteration in 0..100 {
        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..grid_grid.len() {
            //select row
            for j in 0..grid_grid[i].len() {
                //column
                grid_grid[i][j] += 1;
                if grid_grid[i][j] > 9 {
                    stack.push_front((i, j));
                }
            }
        }
        while !stack.is_empty() {
            let (i, j) = stack.pop_front().unwrap();
            seen.insert((i, j));
            for (deltax, deltay) in &directions {
                if (i == 0 && *deltay == -1)
                    || (j == 0 && *deltax == -1)
                    || (i >= grid_grid.len() - 1 && *deltay == 1)
                    || (j >= grid_grid[0].len() - 1 && *deltax == 1)
                {
                    continue;
                }
                let new_x: usize = (j as i32 + deltax).try_into().unwrap();
                let new_y: usize = (i as i32 + deltay).try_into().unwrap();
                grid_grid[new_y][new_x] += 1;
                if grid_grid[new_y][new_x] > 9
                    && !seen.contains(&(new_y, new_x))
                    && !stack.contains(&(new_y, new_x))
                {
                    stack.push_front((new_y, new_x));
                }
            }
        }
        for i in 0..grid_grid.len() {
            //select row
            for j in 0..grid_grid[i].len() {
                //column
                if grid_grid[i][j] > 9 {
                    count += 1;
                    grid_grid[i][j] = 0;
                }
            }
        }
    }
    println!("Silver star: {:?}", count);
}

fn gold() {
    let input = include_str!("input.txt");
    let grid: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut grid_grid: Vec<Vec<i32>> = vec![];
    let directions: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    for element in grid {
        let mut line: Vec<i32> = vec![];
        for element2 in element.chars() {
            line.push(element2.to_digit(10).unwrap().try_into().unwrap());
        }
        grid_grid.push(line)
    }
    let mut iteration = 0;
    loop {
        iteration += 1;
        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..grid_grid.len() {
            //select row
            for j in 0..grid_grid[i].len() {
                //column
                grid_grid[i][j] += 1;
                if grid_grid[i][j] > 9 {
                    stack.push_front((i, j));
                }
            }
        }
        while !stack.is_empty() {
            let (i, j) = stack.pop_front().unwrap();
            seen.insert((i, j));
            for (deltax, deltay) in &directions {
                if (i == 0 && *deltay == -1)
                    || (j == 0 && *deltax == -1)
                    || (i >= grid_grid.len() - 1 && *deltay == 1)
                    || (j >= grid_grid[0].len() - 1 && *deltax == 1)
                {
                    continue;
                }
                let new_x: usize = (j as i32 + deltax).try_into().unwrap();
                let new_y: usize = (i as i32 + deltay).try_into().unwrap();
                grid_grid[new_y][new_x] += 1;
                if grid_grid[new_y][new_x] > 9
                    && !seen.contains(&(new_y, new_x))
                    && !stack.contains(&(new_y, new_x))
                {
                    stack.push_front((new_y, new_x));
                }
            }
        }
        let mut count = 0;
        for i in 0..grid_grid.len() {
            //select row
            for j in 0..grid_grid[i].len() {
                //column
                if grid_grid[i][j] > 9 {
                    count += 1;
                    grid_grid[i][j] = 0;
                }
            }
        }
        if count == 100 {
            println!("Gold star: {}", iteration);
            break;
        }
    }
}

fn main() {
    silver();
    gold();
}

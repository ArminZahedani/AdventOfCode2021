use std::collections::HashSet;
use std::collections::VecDeque;

fn dfs(grid: Vec<Vec<i32>>, point: (usize, usize)) -> u32 {
    let mut deq: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert(point);
    let mut count = 0;
    deq.push_back(point);

    while !deq.is_empty() {
        let new_point: (usize, usize) = deq.pop_front().unwrap();

        if grid[new_point.1][new_point.0] == 9 {
            continue;
        }
        count += 1;
        if new_point.0 != 0 && !seen.contains(&(new_point.0 - 1, new_point.1)) {
            deq.push_back((new_point.0 - 1, new_point.1));
            seen.insert((new_point.0 - 1, new_point.1));
        }
        if new_point.0 != grid[0].len() - 1 && !seen.contains(&(new_point.0 + 1, new_point.1)) {
            deq.push_back((new_point.0 + 1, new_point.1));
            seen.insert((new_point.0 + 1, new_point.1));
        }
        if new_point.1 != 0 && !seen.contains(&(new_point.0, new_point.1 - 1)) {
            deq.push_back((new_point.0, new_point.1 - 1));
            seen.insert((new_point.0, new_point.1 - 1));
        }
        if new_point.1 != grid.len() - 1 && !seen.contains(&(new_point.0, new_point.1 + 1)) {
            deq.push_back((new_point.0, new_point.1 + 1));
            seen.insert((new_point.0, new_point.1 + 1));
        }
    }
    count
}

fn silver() {
    let input = include_str!("input.txt");
    let grid: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut grid_grid: Vec<Vec<i32>> = vec![];
    for element in grid {
        let mut line: Vec<i32> = vec![];
        for element2 in element.chars() {
            line.push(element2.to_digit(10).unwrap().try_into().unwrap());
        }
        grid_grid.push(line)
    }
    let mut lowest_points: Vec<i32> = vec![];
    let mut lowest_points_cords: Vec<(usize, usize)> = vec![];
    for i in 0..grid_grid.len() {
        'outer: for j in 0..grid_grid[0].len() {
            let mut points_to_check: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            if i == 0 {
                points_to_check.retain(|&x| x != (0, -1));
            }
            if j == 0 {
                points_to_check.retain(|&x| x != (-1, 0));
            }
            if i == grid_grid.len() - 1 {
                points_to_check.retain(|&x| x != (0, 1));
            }
            if j == grid_grid[0].len() - 1 {
                points_to_check.retain(|&x| x != (1, 0));
            }
            let mut sentinel: bool = true;
            for element in points_to_check {
                let new_x: usize = (j as i32 + element.0).try_into().unwrap();
                let new_y: usize = (i as i32 + element.1).try_into().unwrap();
                if grid_grid[i][j] >= grid_grid[new_y][new_x] {
                    sentinel = false;
                    continue 'outer;
                }
            }
            if sentinel {
                lowest_points.push(grid_grid[i][j] + 1);
                lowest_points_cords.push((j, i));
            }
        }
    }
    println!("Silver Star: {:?}", lowest_points.iter().sum::<i32>());
    let mut sizes: Vec<u32> = vec![];
    for element in lowest_points_cords {
        sizes.push(dfs(grid_grid.clone(), element));
    }
    sizes.sort();
    println!(
        "Gold star: {:?}",
        sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
    );
}

fn main() {
    silver();
}

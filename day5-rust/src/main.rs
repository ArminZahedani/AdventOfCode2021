fn add_line(p1: Vec<usize>, p2: Vec<usize>, mut grid: Vec<Vec<usize>>, diagonal: bool) -> Vec<Vec<usize>> {
    if p1[1] == p2[1] {
        if p1[0] < p2[0] {
            for i in p1[0]..p2[0] + 1 {
                grid[p1[1]][i] = grid[p1[1]][i] + 1; //grab the correct row based on the y-coordinate. X is i here.
            }
        } else {
            for i in p2[0]..p1[0] + 1 {
                grid[p1[1]][i] = grid[p1[1]][i] + 1;
            }
        }
        return grid;
    } else if p1[0] == p2[0] {
        if p1[1] < p2[1] {
            for j in p1[1]..p2[1] + 1 {
                //y-coordinate
                grid[j][p1[0]] = grid[j][p1[0]] + 1;
            }
        } else {
            for j in p2[1]..p1[1] + 1 {
                grid[j][p1[0]] = grid[j][p1[0]] + 1;
            }
        }
        return grid;
    } else if diagonal {
        //diagonal
        if p1[0] < p2[0] && p1[1] < p2[1] {
            for i in 0..p2[1] - p1[1] + 1 {
                // i is the row number here
                grid[p1[1] + i][p1[0] + i] = grid[p1[1] + i][p1[0] + i] + 1;
            }
        } else if p1[0] < p2[0] && p1[1] > p2[1] {
            for i in 0..p1[1] - p2[1] + 1 {
                grid[p1[1] - i][p1[0] + i] = grid[p1[1] - i][p1[0] + i] + 1;
            }
        } else if p1[0] > p2[0] && p1[1] < p2[1] {
            for i in 0..p2[1] - p1[1] + 1 {
                grid[p1[1] + i][p1[0] - i] = grid[p1[1] + i][p1[0] - i] + 1;
            }
        } else if p1[0] > p2[0] && p1[1] > p2[1] {
            for i in 0..p1[1] - p2[1] + 1 {
                grid[p1[1] - i][p1[0] - i] = grid[p1[1] - i][p1[0] - i] + 1;
            }
        }
    }
    grid
}

fn count_larger_than_2(grid: Vec<Vec<usize>>) -> usize {
    let mut count: usize = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] >= 2 {
                count = count + 1;
            }
        }
    }
    return count;
}

fn main() {
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    let split: Vec<String> = input
        .lines()
        .map(|s| s.trim())
        .map(str::to_string)
        .collect();

    for element in split.iter() {
        let p1_p2 = element.split(" -> ").collect::<Vec<&str>>();
        let p1: Vec<&str> = p1_p2[0].split(",").collect();
        let p2: Vec<&str> = p1_p2[1].split(",").collect();

        let p1: Vec<usize> = p1.iter().map(|s| s.parse().unwrap()).collect();
        let p2: Vec<usize> = p2.iter().map(|s| s.parse().unwrap()).collect();

        grid = add_line(p1, p2, grid, true);
    }
    let count = count_larger_than_2(grid);
    println!("Star: {}", count);
}

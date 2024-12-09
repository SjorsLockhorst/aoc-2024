use std::fs;

const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

fn count_xmas(grid: &Vec<Vec<char>>, position: (usize, usize)) -> u32 {
    let mut total = 0;

    if get_horizontal_forward(grid, position) == XMAS {
        total += 1
    }
    if get_horizontal_backward(grid, position) == XMAS {
        total += 1
    }
    if get_vertical_downward(grid, position) == XMAS {
        total += 1
    }
    if get_vertical_upward(grid, position) == XMAS {
        total += 1
    }
    if get_up_left_diagonal(grid, position) == XMAS {
        total += 1
    }
    if get_up_right_diagonal(grid, position) == XMAS {
        total += 1
    }
    if get_down_left_diagonal(grid, position) == XMAS {
        total += 1
    }
    if get_down_right_diagonal(grid, position) == XMAS {
        total += 1
    }
    return total;
}

fn is_x_mas(grid: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    let (y, x) = position;

    if y == 0 || y == grid.len() - 1 || x == 0 || x == grid[y].len() - 1 {
        return false;
    }

    let mut total = 0;
    let top_left = grid[y - 1][x - 1];
    let top_right = grid[y - 1][x + 1];
    let center = grid[y][x];
    let bottom_left = grid[y + 1][x - 1];
    let bottom_right = grid[y + 1][x + 1];

    let first_diag: Vec<char> = vec![top_left, center, bottom_right];
    let second_diag: Vec<char> = vec![top_right, center, bottom_left];

    if (String::from_iter(&first_diag) == MAS || String::from_iter(first_diag.iter().rev()) == MAS)
        && (String::from_iter(&second_diag) == MAS || String::from_iter(second_diag.iter().rev()) == MAS)
    {
        return true;
    }
    return false;
}

fn get_horizontal_forward(grid: &Vec<Vec<char>>, position: (usize, usize)) -> String {
    let (y, x) = position;
    if (x + XMAS.len()) > grid[y].len() {
        return "".to_string();
    }
    let slice = &grid[y][x..x + XMAS.len()];
    return String::from_iter(slice);
}

fn get_horizontal_backward(grid: &Vec<Vec<char>>, position: (usize, usize)) -> String {
    let (y, x) = position;

    if x < XMAS.len() - 1 {
        return "".to_string();
    }
    let slice = grid[y][x + 1 - XMAS.len()..x + 1].iter().rev();
    return String::from_iter(slice);
}

fn get_vertical_downward(grid: &Vec<Vec<char>>, position: (usize, usize)) -> String {
    let (y, x) = position;

    if y + XMAS.len() > grid.len() {
        return "".to_string();
    }
    let mut chars: Vec<char> = Vec::new();
    for i in 0..XMAS.len() {
        chars.push(grid[y + i][x]);
    }
    return String::from_iter(chars);
}

fn get_vertical_upward(grid: &Vec<Vec<char>>, position: (usize, usize)) -> String {
    let (y, x) = position;

    if y < XMAS.len() - 1 {
        return "".to_string();
    }
    let mut chars: Vec<char> = Vec::new();
    for i in 0..XMAS.len() {
        chars.push(grid[y - i][x]);
    }
    return String::from_iter(chars);
}

fn get_up_left_diagonal(grid: &Vec<Vec<char>>, position: (usize, usize)) -> String {
    let (y, x) = position;

    if y < XMAS.len() - 1 || x < XMAS.len() - 1 {
        return "".to_string();
    }
    let mut chars: Vec<char> = Vec::new();
    for i in 0..XMAS.len() {
        chars.push(grid[y - i][x - i]);
    }
    return String::from_iter(chars);
}

fn get_up_right_diagonal(grid: &Vec<Vec<char>>, position: (usize, usize)) -> String {
    let (y, x) = position;

    if y < XMAS.len() - 1 || x + XMAS.len() > grid.len() {
        return "".to_string();
    }
    let mut chars: Vec<char> = Vec::new();
    for i in 0..XMAS.len() {
        chars.push(grid[y - i][x + i]);
    }
    return String::from_iter(chars);
}

fn get_down_left_diagonal(grid: &Vec<Vec<char>>, position: (usize, usize)) -> String {
    let (y, x) = position;

    if y + XMAS.len() > grid[x].len() || x < XMAS.len() - 1 {
        return "".to_string();
    }
    let mut chars: Vec<char> = Vec::new();
    for i in 0..XMAS.len() {
        chars.push(grid[y + i][x - i]);
    }
    return String::from_iter(chars);
}

fn get_down_right_diagonal(grid: &Vec<Vec<char>>, position: (usize, usize)) -> String {
    let (y, x) = position;

    if y + XMAS.len() > grid[x].len() || x + XMAS.len() > grid.len() {
        return "".to_string();
    }
    let mut chars: Vec<char> = Vec::new();
    for i in 0..XMAS.len() {
        chars.push(grid[y + i][x + i]);
    }
    return String::from_iter(chars);
}

fn part1(grid: &Vec<Vec<char>>) {
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let count = match grid[i][j] {
                'X' => count_xmas(&grid, (i, j)),
                _ => 0,
            };
            total += count;
        }
    }
    println!("{}", total);
}

fn create_grid(contents: String) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    return grid;
}

fn part2(grid: &Vec<Vec<char>>) {
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if match grid[i][j] {
                'A' => is_x_mas(&grid, (i, j)),
                _ => false,
            } {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

// Function to check if a sequence is valid (safe)
pub fn main() {
    let contents = fs::read_to_string("./inputs/day_4.txt").expect("Should be able to find file");
    let grid = create_grid(contents);

    part1(&grid);
    part2(&grid);
}

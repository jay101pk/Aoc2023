use std::collections::HashSet;
use std::io::{self, Write};

advent_of_code::solution!(10);

#[derive(Debug)]
struct GridSpot {
    visited: bool,
    character: char,
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn opposite_direction(d: Direction) -> Direction {
    match d {
        Direction::Down => Direction::Up,
        Direction::Up => Direction::Down,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

fn print_grid(x: usize, y: usize, grid: &Vec<Vec<GridSpot>>) {
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            if x == i && y == j {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("==========");
}

fn follow_pipe(
    previous_direction: Direction,
    x: usize,
    y: usize,
    grid: &Vec<Vec<GridSpot>>,
) -> Option<u32> {
    // print_grid(x, y, grid);

    let a = match grid[y][x].character {
        'S' => Some(0),
        '|' => {
            if previous_direction == Direction::Up {
                follow_pipe(opposite_direction(Direction::Down), x, y + 1, grid)
            } else {
                follow_pipe(opposite_direction(Direction::Up), x, y - 1, grid)
            }
        }
        '-' => {
            if previous_direction == Direction::Left {
                follow_pipe(opposite_direction(Direction::Right), x + 1, y, grid)
            } else {
                follow_pipe(opposite_direction(Direction::Left), x - 1, y, grid)
            }
        }
        'L' => {
            if previous_direction == Direction::Up {
                follow_pipe(opposite_direction(Direction::Right), x + 1, y, grid)
            } else {
                follow_pipe(opposite_direction(Direction::Up), x, y - 1, grid)
            }
        }
        'J' => {
            if previous_direction == Direction::Up {
                follow_pipe(opposite_direction(Direction::Left), x - 1, y, grid)
            } else {
                follow_pipe(opposite_direction(Direction::Up), x, y - 1, grid)
            }
        }
        '7' => {
            if previous_direction == Direction::Down {
                follow_pipe(opposite_direction(Direction::Left), x - 1, y, grid)
            } else {
                follow_pipe(opposite_direction(Direction::Down), x, y + 1, grid)
            }
        }
        'F' => {
            if previous_direction == Direction::Down {
                follow_pipe(opposite_direction(Direction::Right), x + 1, y, grid)
            } else {
                follow_pipe(opposite_direction(Direction::Down), x, y + 1, grid)
            }
        }
        _ => None,
    };

    if let Some(u) = a {
        // println!("{x} {y} {} {u}", grid[y][x].character);

        return Some(u + 1);
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<GridSpot>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| GridSpot {
                    visited: false,
                    character: c,
                })
                .collect()
        })
        .collect();

    let mut x = 0;

    let y = grid
        .iter()
        .position(|s| {
            if let Some(a) = s.iter().position(|t| t.character == 'S') {
                x = a;
                true
            } else {
                false
            }
        })
        .unwrap();

    println!("({x}, {y})");

    let left = if x > 0 {
        match grid[y][x - 1].character {
            '-' | 'F' | 'L' => follow_pipe(opposite_direction(Direction::Left), x - 1, y, &grid),
            _ => None,
        }
    } else {
        None
    };

    let right = if x < grid[0].len() {
        match grid[y][x + 1].character {
            '-' | '7' | 'J' => follow_pipe(opposite_direction(Direction::Right), x + 1, y, &grid),
            _ => None,
        }
        
    } else {
        None
    };

    let up = if y > 0 {
        match grid[y - 1][x].character {
            '|' | 'L' | 'J' => follow_pipe(opposite_direction(Direction::Up), x, y - 1, &grid),
            _ => None,
        }
        
    } else {
        None
    };

    let down = if y < grid.len() {
        match grid[y + 1][x].character {
            '|' | 'F' | '7' => follow_pipe(opposite_direction(Direction::Down), x, y + 1, &grid),
            _ => None,
        }
        
    } else {
        None
    };

    if let Some(mut l) = left {
        println!("left {l}");

        if l % 2 == 1 {
            l += 1
        }
        return Some(l / 2);
    }
    if let Some(mut r) = right {
        println!("right {r}");

        if r % 2 == 1 {
            r += 1
        }
        return Some(r / 2);
    }
    if let Some(mut u) = up {
        println!("up {u}");

        if u % 2 == 1 {
            u += 1
        }
        return Some(u / 2);
    }
    if let Some(mut d) = down {
        println!("down {d}");

        if d % 2 == 1 {
            d += 1
        }
        return Some(d / 2);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

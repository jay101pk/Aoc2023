use std::collections::HashMap;

advent_of_code::solution!(3);

#[derive(Hash, Eq, PartialEq, Debug)]
struct Gear {
    num_parts: usize,
    prod: u32,
}

fn found_gear(x: usize, y: usize, v: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    if let Some((a, b)) = found_symbol(x, y, v) {
        if v[b][a] == '*' {
            return Some((a, b));
        }
    }
    None
}

fn found_symbol(x: usize, y: usize, v: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let x_left_edge = x == 0;
    let x_right_edge = x == v[0].len() - 1;

    let y_top_edge = y == 0;
    let y_bot_edge = y == v.len() - 1;

    if !x_left_edge {
        if is_symbol(v[y][x - 1]) {
            return Some((x - 1, y));
        }
        if !y_top_edge && is_symbol(v[y - 1][x - 1]) {
            return Some((x - 1, y - 1));
        }
        if !y_bot_edge && is_symbol(v[y + 1][x - 1]) {
            return Some((x - 1, y + 1));
        }
    }
    if !x_right_edge {
        if is_symbol(v[y][x + 1]) {
            return Some((x + 1, y));
        }
        if !y_top_edge && is_symbol(v[y - 1][x + 1]) {
            return Some((x + 1, y - 1));
        }
        if !y_bot_edge && is_symbol(v[y + 1][x + 1]) {
            return Some((x + 1, y + 1));
        }
    }
    if !y_top_edge && is_symbol(v[y - 1][x]) {
        return Some((x, y - 1));
    }
    if !y_bot_edge && is_symbol(v[y + 1][x]) {
        return Some((x, y + 1));
    }

    None
}

fn is_symbol(c: char) -> bool {
    !char::is_numeric(c) && c != '.'
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let x = lines.first().unwrap().len();
    let y = lines.len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; x]; y];

    // println!("{:?}", visited);
    let mut sum = 0;
    for i in 0..y {
        for mut j in 0..x {
            // println!("j1 {j}");

            if visited[i][j] {
                continue;
            }

            visited[i][j] = true;
            if char::is_numeric(lines[i][j]) {
                let mut is_part = found_symbol(j, i, &lines).is_some();
                let mut part_number = lines[i][j].to_digit(10).unwrap();

                while j < x - 1 && char::is_numeric(lines[i][j + 1]) {
                    j += 1;
                    visited[i][j] = true;
                    is_part |= found_symbol(j, i, &lines).is_some();
                    part_number *= 10;
                    part_number += lines[i][j].to_digit(10).unwrap();
                }
                // println!("j2 {j}");
                // println!("part number {part_number}");
                if is_part {
                    sum += part_number;
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let x = lines.first().unwrap().len();
    let y = lines.len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; x]; y];

    // println!("{:?}", visited);

    let mut gears: HashMap<(usize, usize), Gear> = HashMap::new();

    let _sum = 0;
    for i in 0..y {
        for mut j in 0..x {
            // println!("j1 {j}");

            if visited[i][j] {
                continue;
            }

            visited[i][j] = true;
            if char::is_numeric(lines[i][j]) {
                let mut gear_pos = found_gear(j, i, &lines);
                let mut part_number = lines[i][j].to_digit(10).unwrap();

                while j < x - 1 && char::is_numeric(lines[i][j + 1]) {
                    j += 1;
                    visited[i][j] = true;
                    let a = found_gear(j, i, &lines);
                    if a.is_some() {
                        gear_pos = a;
                    }
                    part_number *= 10;
                    part_number += lines[i][j].to_digit(10).unwrap();
                }
                // println!("j2 {j}");
                // println!("part number {part_number}");
                if let Some((a, b)) = gear_pos {
                    let _g = gears
                        .entry((a, b))
                        .and_modify(|g| {
                            g.num_parts += 1;
                            g.prod *= part_number;
                        })
                        .or_insert(Gear {
                            num_parts: 1,
                            prod: part_number,
                        });

                    // println!("{:?}", g);
                }
            }
        }
    }
    let sum = gears
        .iter()
        .filter(|(_k, v)| v.num_parts == 2)
        .fold(0, |acc, (_k, v)| acc + v.prod);

    // println!("{:?}", gears);
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}

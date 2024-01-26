use rayon::prelude::*;
use std::collections::HashMap;

use regex::*;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let dir = lines.next().unwrap();

    let r = Regex::new(r"(?<key>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();

    let map: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|l| {
            let c = r.captures(l).unwrap();
            (
                c.name("key").unwrap().as_str(),
                (
                    c.name("left").unwrap().as_str(),
                    c.name("right").unwrap().as_str(),
                ),
            )
        })
        .collect();

    let mut start = "AAA";
    let mut dist = 0;

    while start != "ZZZ" {
        for c in dir.chars() {
            match c {
                'L' => start = map[start].0,
                'R' => start = map[start].1,
                _ => (),
            }

            dist += 1;

            if start == "ZZZ" {
                return Some(dist);
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let dir = lines.next().unwrap();

    let r = Regex::new(r"(?<key>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();

    let map: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|l| {
            let c = r.captures(l).unwrap();
            (
                c.name("key").unwrap().as_str(),
                (
                    c.name("left").unwrap().as_str(),
                    c.name("right").unwrap().as_str(),
                ),
            )
        })
        .collect();

    let starts: Vec<&str> = map
        .keys()
        .filter(|k| ends_with(k, 'A'))
        .map(|&s| s)
        .collect();

    println!("{:?}", starts);

    // return None;

    let a: Vec<u64> = starts
        .par_iter()
        .map(|&s| {
            let mut start = s;
            let mut d = dir.chars().cycle();
            let mut dist = 0;
            loop {
                let c = d.next().unwrap();
                match c {
                    'L' => start = map[start].0,
                    'R' => start = map[start].1,
                    _ => (),
                }
                dist += 1;

                if ends_with(start, 'Z') {
                    return dist;
                }
            }
        })
        .collect();

    let b = a.iter().fold(1, |acc, &v| num::integer::lcm(acc, v));

    println!("{:?}", a);
    Some(b)

    // while !starts.iter().all(|s| ends_with(s, 'Z')) {
    //     for c in dir.chars() {
    //         match c {
    //             'L' => starts = starts.iter().map(|s| map[s].0).collect(),
    //             'R' => starts = starts.iter().map(|s| map[s].1).collect(),
    //             _ => (),
    //         }

    //         dist += 1;

    //         if starts.iter().all(|s| ends_with(s, 'Z')) {
    //             return Some(dist);
    //         }

    //         if dist % 100 ==0 {
    //            println!("{dist} {:?}", starts);
    //         }
    //     }
    // }
}

fn ends_with(s: &str, c: char) -> bool {
    s.chars().nth(2).unwrap() == c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}

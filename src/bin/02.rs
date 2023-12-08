use regex::*;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let green_marbles = 13;
    let red_marbles = 12;
    let blue_marbles = 14;

    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        // println!("{line}");
        let mut bad = false;
        let green = Regex::new(r"(?<num>\d+) green").unwrap();

        let g = green
            .captures_iter(line)
            .map(|caps| caps.name("num").unwrap().as_str().parse::<i32>().unwrap() > green_marbles)
            .any(|x| x);
        // println!("green {g}");
        bad |= g;

        let red = Regex::new(r"(?<num>\d+) red").unwrap();
        let r = red
            .captures_iter(line)
            .map(|caps| caps.name("num").unwrap().as_str().parse::<i32>().unwrap() > red_marbles)
            .any(|x| x);
        // println!("red {r}");
        bad |= r;

        let blue = Regex::new(r"(?<num>\d+) blue").unwrap();
        let b = blue
            .captures_iter(line)
            .map(|caps| caps.name("num").unwrap().as_str().parse::<i32>().unwrap() > blue_marbles)
            .any(|x| x);
        // println!("blue {b}");
        bad |= b;

        if !bad {
            sum += i + 1;
        }
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;
    for line in lines.iter() {
        // println!("{line}");
        let green = Regex::new(r"(?<num>\d+) green").unwrap();

        let g = green
            .captures_iter(line)
            .map(|caps| caps.name("num").unwrap().as_str().parse::<i32>().unwrap())
            .max()
            .unwrap();
        // println!("green {g}");

        let red = Regex::new(r"(?<num>\d+) red").unwrap();
        let r = red
            .captures_iter(line)
            .map(|caps| caps.name("num").unwrap().as_str().parse::<i32>().unwrap())
            .max()
            .unwrap();
        // println!("red {r}");

        let blue = Regex::new(r"(?<num>\d+) blue").unwrap();
        let b = blue
            .captures_iter(line)
            .map(|caps| caps.name("num").unwrap().as_str().parse::<i32>().unwrap())
            .max()
            .unwrap();
        // println!("blue {b}");

        sum += b * g * r;
    }
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

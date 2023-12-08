advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());
    let distances = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap());

    let p: u32 = times
        .zip(distances)
        .map(|(t, d)| {
            for i in 1..t / 2 {
                if (t - i) * i > d {
                    let a = (((t + 1) / 2) - i) * 2;
                    if t % 2 == 0 {
                        return a + 1;
                    } else {
                        return a;
                    }
                }
            }
            1
        })
        .product();

    Some(p)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let time: u64 = lines[0]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = lines[1]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse()
        .unwrap();

    for i in 1..time / 2 {
        if (time - i) * i > distance {
            let a = (((time + 1) / 2) - i) * 2;
            if time % 2 == 0 {
                return Some(a + 1);
            } else {
                return Some(a);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}

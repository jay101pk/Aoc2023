use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut sum = 0;
    for l in lines {
        let mut num = l.split(':').nth(1).unwrap().split('|');
        let win = num.next().unwrap();
        let wins: HashSet<u32> = win
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let your = num.next().unwrap();
        let yours: HashSet<u32> = your
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        // println!("{:?} {:?}", wins, yours);
        let winners: Vec<&u32> = wins.intersection(&yours).collect();
        let winners_count: u32 = winners.len().try_into().unwrap();
        if winners_count > 0 {
            sum += u32::pow(2, winners_count - 1);
        }

        // println!("{:?}", winners);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut cards = vec![1; lines.len()];

    for i in 0..lines.len() {
        let l = lines[i];
        let mut num = l.split(':').nth(1).unwrap().split('|');
        let win = num.next().unwrap();
        let wins: HashSet<i32> = win
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let your = num.next().unwrap();
        let yours: HashSet<i32> = your
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        // println!("{:?} {:?}", wins, yours);

        let winners: Vec<&i32> = wins.intersection(&yours).collect();
        let winners_count = winners.len();

        for j in 1..winners_count + 1 {
            cards[i + j] += cards[i];
        }
    }
    let card_count: u32 = cards.into_iter().sum();
    Some(card_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

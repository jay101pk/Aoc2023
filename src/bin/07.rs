use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bet: u32,
    joker: bool,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let s = score_hand(&self.cards, self.joker);
        let o = score_hand(&other.cards, self.joker);
        if s > o {
            return Some(Ordering::Greater);
        }
        if s < o {
            return Some(Ordering::Less);
        }
        if self.cards == other.cards {
            return Some(Ordering::Equal);
        }
        let a = self.cards.iter().zip(&other.cards).find_map(|(c1, c2)| {
            let s1 = score_label(*c1, self.joker);
            let s2 = score_label(*c2, other.joker);
            if s1 > s2 {
                return Some(Ordering::Greater);
            }
            if s2 > s1 {
                return Some(Ordering::Less);
            }
            None
        });
        if a.is_some() {
            return a;
        }

        Some(Ordering::Equal)
    }
}

fn score_hand(cards: &[char], joker: bool) -> u32 {
    let mut m: HashMap<char, u32> = HashMap::new();

    for card in cards {
        *m.entry(*card).or_insert(0) += 1;
    }

    if cards
        .iter()
        .collect::<String>()
        .cmp(&String::from("JJJJJ"))
        .is_eq()
    {
        return 6;
    }

    if joker && m.contains_key(&'J') {
        let b = m.remove(&'J').unwrap();

        let n = m.values_mut().max();

        *n.unwrap() += b;
    }

    if m.values().any(|&x| x == 5) {
        return 6;
    } else if m.values().any(|&x| x == 4) {
        return 5;
    } else if m.values().any(|&x| x == 3) {
        return if m.values().any(|&x| x == 2) { 4 } else { 3 };
    }
    m.values().filter(|&&x| x == 2).count() as u32
}

fn score_label(c: char, joker: bool) -> u32 {
    if joker {
        return match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 11,
            '9' => 10,
            '8' => 9,
            '7' => 8,
            '6' => 7,
            '5' => 6,
            '4' => 5,
            '3' => 4,
            '2' => 3,
            '1' => 2,
            _ => 0,
        };
    }

    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        '1' => 1,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|s| {
            let mut a = s.split_whitespace();
            let cards = a.next().unwrap();
            let bet = a.next().unwrap();
            Hand {
                cards: cards.chars().collect(),
                bet: bet.parse().unwrap(),
                joker: false,
            }
        })
        .collect();

    hands.sort_by(|a, b| Hand::partial_cmp(a, b).unwrap());

    // println!("{:?}", hands);

    Some(
        hands
            .into_iter()
            .enumerate()
            .fold(0, |a, h| a + ((h.0 + 1) as u32 * h.1.bet)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|s| {
            let mut a = s.split_whitespace();
            let cards = a.next().unwrap();
            let bet = a.next().unwrap();
            Hand {
                cards: cards.chars().collect(),
                bet: bet.parse().unwrap(),
                joker: true,
            }
        })
        .collect();

    hands.sort_by(|a, b| Hand::partial_cmp(a, b).unwrap());

    // println!("{:?}", hands);

    Some(
        hands
            .into_iter()
            .enumerate()
            .fold(0, |a, h| a + ((h.0 + 1) as u32 * h.1.bet)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}

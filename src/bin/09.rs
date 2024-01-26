advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let rows = input.lines();

    let sum: i64 = rows
        .map(|l| {
            let mut sequence: Vec<i64> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
            let mut lasts = vec![];

            loop {
                let last = *sequence.last().unwrap();

                if last == 0 {
                    break;
                }

                lasts.push(last);

                let a = sequence.iter().take(sequence.len() - 1);

                let b = sequence.iter().skip(1);

                sequence = a.zip(b).map(|(x, y)| y - x).collect();
            }

            // println!("{:?}", lasts);

            lasts.iter().sum::<i64>()
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let rows = input.lines();

    let sum: i64 = rows
        .map(|l| {
            let sequence: Vec<i64> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();

            rec_diff(&sequence)
        })
        .sum();

    Some(sum)
}

fn rec_diff(v: &Vec<i64>) -> i64 {
    if v.iter().all(|&x| x == 0) {
        return 0;
    }

    let a = v.iter().take(v.len() - 1);

    let b = v.iter().skip(1);

    let y: Vec<i64> = a.zip(b).map(|(x, y)| y - x).collect();

    let z = rec_diff(&y);

    // println!("{z} {:?}", y);

    v.first().unwrap() - z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

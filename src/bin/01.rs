advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let d: Vec<&str> = line.matches(char::is_numeric).collect();
        let mut a = String::new();
        a.push(d.first().unwrap().parse::<char>().unwrap());
        a.push(d.last().unwrap().parse::<char>().unwrap());
        sum += a.parse::<u32>().unwrap();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut s = line.to_string();
        s = s.replace("zero", "zero0zero");
        s = s.replace("one", "one1one");
        s = s.replace("two", "two2two");
        s = s.replace("three", "three3three");
        s = s.replace("four", "four4four");
        s = s.replace("five", "five5five");
        s = s.replace("six", "six6six");
        s = s.replace("seven", "seven7seven");
        s = s.replace("eight", "eight8eight");
        s = s.replace("nine", "nine9nine");
        let d: Vec<&str> = s.matches(char::is_numeric).collect();
        let mut a = String::new();
        a.push(d.first().unwrap().parse::<char>().unwrap());
        a.push(d.last().unwrap().parse::<char>().unwrap());
        sum += a.parse::<u32>().unwrap();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}

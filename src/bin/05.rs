use rayon::prelude::*;

advent_of_code::solution!(5);

#[derive(Debug)]
struct RangeMap {
    start: u64,
    num: u64,
    out: u64,
}

impl RangeMap {
    fn get_map(&self, number: u64) -> Option<u64> {
        if number < self.start {
            return None;
        }

        let n = number - self.start;

        if n < self.num {
            return Some(self.out + n);
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let seeds: Vec<u64> = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{:?}", seeds);

    let mut it = lines.iter();

    let soil_index = it.position(|&s| s == "seed-to-soil map:").unwrap();
    let fert_index = it.position(|&s| s == "soil-to-fertilizer map:").unwrap() + soil_index + 1;
    let water_index = it.position(|&s| s == "fertilizer-to-water map:").unwrap() + fert_index + 1;
    let light_index = it.position(|&s| s == "water-to-light map:").unwrap() + water_index + 1;
    let temp_index = it.position(|&s| s == "light-to-temperature map:").unwrap() + light_index + 1;
    let humid_index = it
        .position(|&s| s == "temperature-to-humidity map:")
        .unwrap()
        + temp_index
        + 1;
    let loc_index = it.position(|&s| s == "humidity-to-location map:").unwrap() + humid_index + 1;

    let mut soil_map: Vec<RangeMap> = vec![];
    let mut fert_map: Vec<RangeMap> = vec![];
    let mut water_map: Vec<RangeMap> = vec![];
    let mut light_map: Vec<RangeMap> = vec![];
    let mut temp_map: Vec<RangeMap> = vec![];
    let mut humid_map: Vec<RangeMap> = vec![];
    let mut loc_map: Vec<RangeMap> = vec![];

    for line in lines.iter().take(fert_index - 1).skip(soil_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        soil_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    for line in lines.iter().take(water_index - 1).skip(fert_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        fert_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    for line in lines.iter().take(light_index - 1).skip(water_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        water_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    for line in lines.iter().take(temp_index - 1).skip(light_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        light_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }
    for line in lines.iter().take(humid_index - 1).skip(temp_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        temp_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    for line in lines.iter().take(loc_index - 1).skip(humid_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        humid_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }
    for line in lines.iter().skip(loc_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        loc_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    // println!("{} {:?}", loc_map.len(), loc_map);

    let min_dist = seeds
        .iter()
        .map(|s| {
            // println!("{s}");
            let sm = soil_map.iter().find_map(|m| m.get_map(*s)).unwrap_or(*s);
            // println!("{sm}");
            let fm = fert_map.iter().find_map(|m| m.get_map(sm)).unwrap_or(sm);
            // println!("{fm}");
            let wm = water_map.iter().find_map(|m| m.get_map(fm)).unwrap_or(fm);
            // println!("{wm}");
            let lm = light_map.iter().find_map(|m| m.get_map(wm)).unwrap_or(wm);
            // println!("{lm}");
            let tm = temp_map.iter().find_map(|m| m.get_map(lm)).unwrap_or(lm);
            // println!("{tm}");
            let hm = humid_map.iter().find_map(|m| m.get_map(tm)).unwrap_or(tm);
            // println!("{hm}");
            let a = loc_map.iter().find_map(|m| m.get_map(hm)).unwrap_or(hm);
            // println!("{a}");
            // println!();
            a
        })
        .min()
        .unwrap();
    Some(min_dist)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let seeds: Vec<u64> = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{:?}", seeds);

    let mut it = lines.iter();

    let soil_index = it.position(|&s| s == "seed-to-soil map:").unwrap();
    let fert_index = it.position(|&s| s == "soil-to-fertilizer map:").unwrap() + soil_index + 1;
    let water_index = it.position(|&s| s == "fertilizer-to-water map:").unwrap() + fert_index + 1;
    let light_index = it.position(|&s| s == "water-to-light map:").unwrap() + water_index + 1;
    let temp_index = it.position(|&s| s == "light-to-temperature map:").unwrap() + light_index + 1;
    let humid_index = it
        .position(|&s| s == "temperature-to-humidity map:")
        .unwrap()
        + temp_index
        + 1;
    let loc_index = it.position(|&s| s == "humidity-to-location map:").unwrap() + humid_index + 1;

    let mut soil_map: Vec<RangeMap> = vec![];
    let mut fert_map: Vec<RangeMap> = vec![];
    let mut water_map: Vec<RangeMap> = vec![];
    let mut light_map: Vec<RangeMap> = vec![];
    let mut temp_map: Vec<RangeMap> = vec![];
    let mut humid_map: Vec<RangeMap> = vec![];
    let mut loc_map: Vec<RangeMap> = vec![];

    for line in lines.iter().take(fert_index - 1).skip(soil_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        soil_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    for line in lines.iter().take(water_index - 1).skip(fert_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        fert_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    for line in lines.iter().take(light_index - 1).skip(water_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        water_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    for line in lines.iter().take(temp_index - 1).skip(light_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        light_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }
    for line in lines.iter().take(humid_index - 1).skip(temp_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        temp_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    for line in lines.iter().take(loc_index - 1).skip(humid_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        humid_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }
    for line in lines.iter().skip(loc_index + 1) {
        let v: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        loc_map.push(RangeMap {
            start: v[1],
            num: v[2],
            out: v[0],
        });
    }

    // println!("{} {:?}", loc_map.len(), loc_map);

    let mut min_dist = u64::MAX;

    for i in (0..seeds.len()).step_by(2) {
        let seed_start = seeds[i];
        let seed_range = seeds[i + 1];
        println!("{seed_start} {seed_range}");

        let md = (seed_start..seed_start + seed_range)
            .into_par_iter()
            .map(|j| {
                let sm = soil_map.iter().find_map(|m| m.get_map(j)).unwrap_or(j);
                // println!("{sm}");
                let fm = fert_map.iter().find_map(|m| m.get_map(sm)).unwrap_or(sm);
                // println!("{fm}");
                let wm = water_map.iter().find_map(|m| m.get_map(fm)).unwrap_or(fm);
                // println!("{wm}");
                let lm = light_map.iter().find_map(|m| m.get_map(wm)).unwrap_or(wm);
                // println!("{lm}");
                let tm = temp_map.iter().find_map(|m| m.get_map(lm)).unwrap_or(lm);
                // println!("{tm}");
                let hm = humid_map.iter().find_map(|m| m.get_map(tm)).unwrap_or(tm);
                // println!("{hm}");
                let a = loc_map.iter().find_map(|m| m.get_map(hm)).unwrap_or(hm);
                // println!("{a}");
                // println!();
                a
            })
            .min()
            .unwrap();

        min_dist = min_dist.min(md);
    }

    Some(min_dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}

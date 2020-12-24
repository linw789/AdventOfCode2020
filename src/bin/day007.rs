use std::str::from_utf8;
use std::vec::Vec;
use itertools::Itertools;
use std::collections::HashMap;

fn part_1(rule_map: &HashMap<&str, Vec<(&str, usize)>>) -> i32 {
    fn contains_shiny_gold(id: &str, rule_map: &HashMap<&str, Vec<(&str, usize)>>) -> bool {
        match rule_map.get(id) {
            Some(bags) => {
                for (id, _) in bags {
                    if *id == "shiny gold" {
                        return true;
                    } else {
                        if contains_shiny_gold(id, rule_map) {
                            return true;
                        }
                    }
                }
                return false;
            },
            None => { return false; },
        };
    }

    let mut sum = 0;
    for id in rule_map.keys() {
        if *id == "shiny gold" {
            continue;
        }
        if contains_shiny_gold(id, rule_map) {
            sum += 1;
        }
    }

    return sum;
}

fn part_2(rule_map: &HashMap<&str, Vec<(&str, usize)>>) -> i32 {
    fn total_bags(id: &str, rule_map: &HashMap<&str, Vec<(&str, usize)>>) -> i32 {
        let mut sum = 1;

        match rule_map.get(id) {
            Some(contents) => {
                sum += contents.iter().fold(0, |acc, idn| { return acc + (idn.1 as i32) * total_bags(idn.0, rule_map); });
            },
            None => {},
        };

        return sum;
    }

    return total_bags("shiny gold", rule_map) - 1;
}

fn main() {
    let input = include_bytes!("day007.input");
    let rule_lines = from_utf8(input).unwrap().lines();

    let mut rule_map: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();

    for line in rule_lines {
        let (id, mut remainder) = line.splitn(2, " bags contain ").next_tuple().unwrap();
        let mut contents: Vec<(&str, usize)> = Vec::new();

        remainder = remainder.trim_start();
        while remainder.starts_with(char::is_numeric) {
            let (n, sub_remainder) = remainder.split_at(remainder.find(|c: char| !c.is_numeric()).unwrap());
            let bagn = n.trim().parse::<usize>().unwrap();

            remainder = sub_remainder;
            let (id, sub_remainder) = remainder.splitn(2, "bag").next_tuple().unwrap();
            let id = id.trim();

            contents.push((id, bagn));

            remainder = sub_remainder
                .trim_start_matches("bag")
                .trim_start_matches("s")
                .trim_start_matches(",")
                .trim_start();
        }

        rule_map.insert(id, contents);
    }

    println!("Part 1, number of bags containing shiny gold bag: {}", part_1(&rule_map));

    println!("Part 2, number of bags in shiny gold bag: {}", part_2(&rule_map));
}

use itertools::Itertools;
use std::str::from_utf8;
use std::vec::Vec;

#[derive(Clone, Debug)]
enum Rule {
    End(char),
    Next(Vec<usize>),
    Choice((Vec<usize>, Vec<usize>)),
    Invalid,
}

fn matches<'a>(msg: &'a str, rules: &[Rule], rule_id: usize) -> (&'a str, bool) {
    fn matches_subrule<'a>(msg: &'a str, subrule: &Vec<usize>, rules: &[Rule]) -> (&'a str, bool) {
        let mut remaining_str = msg;
        let mut matched = false;
        for id in subrule {
            let (rs, subrule_matched) = matches(remaining_str, rules, *id);
            matched = subrule_matched;
            if matched {
                remaining_str = rs;
            } else {
                break;
            }
        }
        if matched {
            return (remaining_str, matched);
        } else {
            return (msg, matched);
        }
    }

    let mut remaining_str = msg;
    let mut matched;
    match &rules[rule_id] {
        Rule::End(letter) => {
            // println!("letter: {}, remaining: {}", *letter, remaining_str);
            if !msg.is_empty() && *letter == msg.chars().next().unwrap() {
                remaining_str = &msg[1..];
                matched = true;
            } else {
                matched = false;
            }
        }
        Rule::Next(subrule) => {
            let (rs, subrule_matched) = matches_subrule(remaining_str, subrule, rules);
            remaining_str = rs;
            matched = subrule_matched;
        }
        Rule::Choice((subrule0, subrule1)) => {
            let (rs, subrule_matched) = matches_subrule(remaining_str, subrule0, rules);
            remaining_str = rs;
            matched = subrule_matched;

            if matched == false {
                let (rs, subrule_matched) = matches_subrule(remaining_str, subrule1, rules);
                remaining_str = rs;
                matched = subrule_matched;
            }
        }
        Rule::Invalid => {
            panic!("Invalid rule.");
        }
    }

    return (remaining_str, matched);
}

fn part_1(rules: &[Rule], messages: &[&str]) -> usize {
    return messages
        .iter()
        .filter(|msg| {
            let (remaining_str, matched) = matches(msg, rules, 0);
            // println!("matched: {}, remaining: {}", matched, remaining_str);
            return remaining_str.is_empty() && matched;
        })
        .count();
}

fn main() {
    let input = include_bytes!("day019.input");
    let lines = from_utf8(input).unwrap().lines();

    let mut rules = Vec::new();
    for line in lines.clone().take_while(|line| !line.is_empty()) {
        let (id, rule) = line.split(':').next_tuple().unwrap();
        let id = id.parse::<usize>().unwrap();

        if id >= rules.len() {
            rules.resize(id + 1, Rule::Invalid);
        }

        let rule = rule.trim();
        if let Some(letter) = rule.strip_prefix('"') {
            rules[id] = Rule::End(letter.chars().next().unwrap());
        } else if let Some((rule_opt0, rule_opt1)) = rule.split('|').next_tuple() {
            rules[id] = Rule::Choice((
                rule_opt0
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
                rule_opt1
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            ));
        } else {
            rules[id] = Rule::Next(
                rule.split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }
    }

    let messages: Vec<&str> = lines.skip_while(|line| !line.is_empty()).skip(1).collect();

    println!("Part 1: {}", part_1(&rules, &messages));
}

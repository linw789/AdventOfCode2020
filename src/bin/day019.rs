use itertools::Itertools;
use std::str::from_utf8;
use std::vec::Vec;

#[derive(Clone, Debug)]
enum Rule {
    End(char),
    Next(Vec<usize>),
    Choice(Vec<usize>, Vec<usize>),
    Invalid,
}

fn matches<'a>(msgs: &Vec<&'a str>, rules: &[Rule], rule_id: usize) -> Vec<&'a str> {
    #[inline]
    fn matches_subrule<'a>(msgs: &Vec<&'a str>, subrule: &[usize], rules: &[Rule]) -> Vec<&'a str> {
        let mut possible_remaining_msgs = msgs.clone();
        for id in subrule {
            possible_remaining_msgs = matches(&possible_remaining_msgs, rules, *id);
            if possible_remaining_msgs.is_empty() {
                break;
            }
        }
        return possible_remaining_msgs;
    }

    let mut possible_remaining_msgs = Vec::new();
    match &rules[rule_id] {
        Rule::End(letter) => {
            for msg in msgs {
                if !msg.is_empty() && *letter == msg.chars().next().unwrap() {
                    possible_remaining_msgs.push(&msg[1..]);
                }
            }
        }
        Rule::Next(subrule) => {
            possible_remaining_msgs = matches_subrule(msgs, subrule, rules);
        }
        Rule::Choice(subrule0, subrule1) => {
            let mut remaining_msgs = matches_subrule(msgs, subrule0, rules);
            possible_remaining_msgs.append(&mut remaining_msgs);

            let mut remaining_msgs = matches_subrule(msgs, subrule1, rules);
            possible_remaining_msgs.append(&mut remaining_msgs);
        }
        Rule::Invalid => {
            panic!("Invalid rule.");
        }
    }

    return possible_remaining_msgs;
}

fn part_1(rules: &[Rule], messages: &[&str]) -> usize {
    return messages
        .iter()
        .filter(|msg| {
            let remaining_msgs = matches(&(vec![msg]), rules, 0);
            let mut res = false;
            for msg in remaining_msgs {
                if msg.len() == 0 {
                    res = true;
                }
            }
            return res;
        })
        .count();
}

fn part_2(rules: &[Rule], messages: &[&str]) -> usize {
    let mut rules = rules.to_vec();
    rules[8] = Rule::Choice(vec![42], vec![42, 8]);
    rules[11] = Rule::Choice(vec![42, 31], vec![42, 11, 31]);
    return messages
        .iter()
        .filter(|msg| {
            let remaining_msgs = matches(&(vec![msg]), &rules, 0);
            let mut res = false;
            // For part 2, there will 
            for msg in remaining_msgs {
                if msg.len() == 0 {
                    res = true;
                }
            }
            return res;
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
            rules[id] = Rule::Choice(
                rule_opt0
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
                rule_opt1
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
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
    println!("Part 2: {}", part_2(&rules, &messages));
}

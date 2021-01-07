use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Range;
use std::str::from_utf8;
use std::vec::Vec;

struct Rule<'a> {
    pub name: &'a str,
    range0: Range<u32>,
    range1: Range<u32>,
}

impl<'a> Rule<'a> {
    pub fn new(name: &'a str, min0: u32, max0: u32, min1: u32, max1: u32) -> Self {
        return Self {
            name,
            range0: (min0..max0 + 1),
            range1: (min1..max1 + 1),
        };
    }

    pub fn validate(&self, v: u32) -> bool {
        return self.range0.contains(&v) || self.range1.contains(&v);
    }
}

fn main() {
    let input = include_bytes!("day016.input");
    let (rules_str, tickets) = from_utf8(input)
        .unwrap()
        .splitn(2, "your ticket:")
        .next_tuple()
        .unwrap();
    let (my_ticket, nearby_tickets) = tickets.splitn(2, "nearby tickets:").next_tuple().unwrap();
    let my_ticket_str = my_ticket.trim();
    let nearby_tickets_str = nearby_tickets.trim();

    // Parse rules.
    let mut rules = Vec::new();
    for line in rules_str.trim().lines() {
        let (rule_name, remainder) = line.split(":").next_tuple().unwrap();
        let (rule0, rule1) = remainder.split("or").next_tuple().unwrap();
        let (min0, max0) = rule0.split("-").next_tuple().unwrap();
        let (min1, max1) = rule1.split("-").next_tuple().unwrap();
        rules.push(Rule::new(
            rule_name,
            min0.trim().parse::<u32>().unwrap(),
            max0.trim().parse::<u32>().unwrap(),
            min1.trim().parse::<u32>().unwrap(),
            max1.trim().parse::<u32>().unwrap(),
        ))
    }

    let my_ticket: Vec<u32> = my_ticket_str
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // Parse nearby tickets.
    let mut nearby_tickets = Vec::new();
    for line in nearby_tickets_str.lines() {
        let values: Vec<u32> = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
        nearby_tickets.push(values);
    }

    let mut valid_nearby_tickets = Vec::new();

    // Part 1.
    let mut invalid_sum = 0;
    for ticket in &nearby_tickets {
        let mut has_invalid_val = false;
        for val in ticket {
            let mut any_valid = false;
            for rule in &rules {
                if rule.validate(*val) {
                    any_valid = true;
                }
            }
            if !any_valid {
                invalid_sum += val;
                has_invalid_val = true;
            }
        }

        if !has_invalid_val {
            valid_nearby_tickets.push(ticket.clone());
        }
    }

    println!("Part 1: {}", invalid_sum);

    // Part 2.

    let mut fields = Vec::new();

    for _ in 0..my_ticket.len() {
        let mut possible_rules_per_field = HashSet::new();

        for rule in &rules {
            possible_rules_per_field.insert(rule.name);
        }
        fields.push(possible_rules_per_field);
    }

    for i in 0..my_ticket.len() {
        for ticket in &valid_nearby_tickets {
            // Remove rules that's impossible for that field.
            for rule in &rules {
                if !rule.validate(ticket[i]) {
                    fields[i].remove(rule.name);
                    break;
                }
            }
        }
    }

    // If a field only contains one possible rule, then the said rule is decided to belong to that
    // field.
    let mut decided_rules = HashSet::new();

    loop {
        for field_possible_rules in &fields {
            if field_possible_rules.len() == 1 {
                decided_rules.insert(*field_possible_rules.iter().next().unwrap());
            }
        }

        if decided_rules.len() == my_ticket.len() {
            break;
        }

        // Remove already decided rules from possible rules of other fields.
        for field_possible_rules in &mut fields {
            if field_possible_rules.len() != 1 {
                for decided_rule in &decided_rules {
                    field_possible_rules.remove(decided_rule);
                }
            }
        }
    }

    let mut depart_product: u64 = 1;

    for (field_name, value) in fields
        .iter()
        .map(|s| *s.iter().next().unwrap())
        .zip(my_ticket.iter().map(|v| *v as u64))
    {
        if field_name.starts_with("departure") {
            depart_product *= value;
        }
    }

    println!("Part 2: {}", depart_product);
}

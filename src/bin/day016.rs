use itertools::Itertools;
use std::str::from_utf8;
use std::vec::Vec;
use std::collections::HashMap;

struct Rule<'a> {
    pub name: &'a str,
    range0: (u32, u32),
    range1: (u32, u32),
}

impl<'a> Rule<'a> {
    pub fn new(name: &'a str, min0: u32, max0: u32, min1: u32, max1: u32) -> Self {
        return Self {
            name,
            range0: (min0, max0),
            range1: (min1, max1),
        };
    }

    pub fn validate(&self, v: u32) -> bool {
        if (v >= self.range0.0 && v <= self.range0.1) || 
           (v >= self.range1.0 && v <= self.range1.1)
        {
            return true;
        } else {
            return false;
        }
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

    let my_ticket: Vec<u32> = my_ticket_str.split(",").map(|s| s.parse::<u32>().unwrap()).collect();

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

    // Test
    let mut valid_field_vector = HashMap::new();
    for rule in &rules {
        valid_field_vector.insert(rule.name, 0u32);
    }

    let mut field_name_order = Vec::new();
    let rules_len = rules.len();
    let total_nearby_tickets = valid_nearby_tickets.len() as u32;
    println!("total nearby tickets: {}", total_nearby_tickets);
    for i in 0..rules_len {
        println!("checking pos: {}", i);
        for ticket in &valid_nearby_tickets {
            for rule in &rules {
                if rule.validate(ticket[i]) {
                    *(valid_field_vector.get_mut(rule.name).unwrap()) += 1;
                }
            }
        }

        println!("valid field: {:?}", valid_field_vector);
        let field_name = valid_field_vector.iter().find_map(|(&k, &v)| {
            if v == total_nearby_tickets {
                return Some(k);
            } else {
                return None;
            }
        }).unwrap();

        field_name_order.push(field_name);

        for (_, v) in valid_field_vector.iter_mut() {
            *v = 0;
        }
    }

    let mut departure_product = 1;
    for (i, field_name) in field_name_order.iter().enumerate() {
        if field_name.starts_with("departure") {
            departure_product *= my_ticket[i];
        }
    }

    println!("Part 2: {}", departure_product);
}

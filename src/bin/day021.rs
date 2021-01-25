use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::str::from_utf8;
use std::vec::Vec;

fn main() {
    let input = include_bytes!("day021.input");

    let mut aller_ingre_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredient_list: Vec<HashSet<&str>> = Vec::new();

    for line in from_utf8(input).unwrap().lines() {
        let (ingredients, allergens) = line.split("(contains").next_tuple().unwrap();
        let ingredients: HashSet<&str> = ingredients.split_whitespace().collect();
        ingredient_list.push(ingredients.clone());

        for allergen in allergens.strip_suffix(')').unwrap().split(",") {
            let allergen = allergen.trim();
            match aller_ingre_map.entry(allergen) {
                Entry::Occupied(mut entry) => {
                    let existing = entry.get_mut();
                    existing.retain(|&v| ingredients.contains(v));
                }
                Entry::Vacant(entry) => {
                    entry.insert(ingredients.clone());
                }
            }
        }
    }

    let mut matched_ingredients: HashSet<&str> = HashSet::new();
    loop {
        if aller_ingre_map.is_empty() {
            break;
        }

        let mut to_process: Vec<(&str, &str)> = Vec::new();
        for (allergen, ingredients) in aller_ingre_map.iter() {
            if ingredients.len() == 1 {
                to_process.push((allergen, ingredients.iter().next().unwrap()));
                matched_ingredients.insert(ingredients.iter().next().unwrap());
            }
        }

        for (allergen, _) in &to_process {
            aller_ingre_map.remove(allergen);
        }

        for (_, ingredients) in aller_ingre_map.iter_mut() {
            for (_, ingredient) in &to_process {
                ingredients.remove(ingredient);
            }
        }
    }

    let mut total: usize = 0;
    for ingredients in &ingredient_list {
        total += ingredients.iter().fold(0, |mut acc, ingredient| {
            if !matched_ingredients.contains(ingredient) {
                acc += 1;
            }
            return acc;
        });
    }

    println!("Part 1: {}", total);
}

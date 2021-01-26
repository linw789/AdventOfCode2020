use itertools::Itertools;
use std::collections::VecDeque;
use std::str::from_utf8;
use std::vec::Vec;

fn part_1(player1: &VecDeque<u32>, player2: &VecDeque<u32>) -> u32 {
    let mut players = [player1.clone(), player2.clone()];

    let mut winner = 0;
    while !players[0].is_empty() && !players[1].is_empty() {
        /*
        for (i, deck) in players.iter().enumerate() {
            println!("deck{}({}): {:?}", i, deck.len(), deck);
        }
        println!("\n");
        */

        let card0 = players[0].pop_front().unwrap();
        let card1 = players[1].pop_front().unwrap();

        let bigger;
        let smaller;

        if card0 > card1 {
            winner = 0;
            bigger = card0;
            smaller = card1;
        } else if card1 > card0 {
            winner = 1;
            bigger = card1;
            smaller = card0;
        } else {
            panic!("Invalid state.");
        }

        players[winner].push_back(bigger);
        players[winner].push_back(smaller);
    }

    let mut sum = 0;
    for (i, val) in players[winner].iter().rev().enumerate() {
        sum += *val * (i as u32 + 1);
    }
    return sum;
}

fn check_dupe_states(
    prev_states: &[(VecDeque<u32>, VecDeque<u32>)],
    deck1: &VecDeque<u32>,
    deck2: &VecDeque<u32>,
) -> bool {
    if prev_states.is_empty() {
        return false;
    }
    for (prev_deck1, prev_deck2) in prev_states {
        if prev_deck1 == deck1 && prev_deck2 == deck2 {
            return true;
        }
    }
    return false;
}

fn recursive_combat(decks: &mut [VecDeque<u32>; 2]) -> usize {
    let mut prev_states = Vec::new();
    let mut winner: usize = 0;
    loop {
        if decks[0].is_empty() || decks[1].is_empty() {
            break;
        }

        if check_dupe_states(&prev_states, &decks[0], &decks[1]) {
            winner = 0;
            break;
        }

        prev_states.push((decks[0].clone(), decks[1].clone()));
        let card0 = decks[0].pop_front().unwrap();
        let card1 = decks[1].pop_front().unwrap();

        if decks[0].len() >= card0 as usize && decks[1].len() >= card1 as usize {
            let mut new_decks = [
                decks[0].make_contiguous()[0..card0 as usize]
                    .iter()
                    .copied()
                    .collect::<VecDeque<u32>>(),
                decks[1].make_contiguous()[0..card1 as usize]
                    .iter()
                    .copied()
                    .collect::<VecDeque<u32>>(),
            ];
            winner = recursive_combat(&mut new_decks);
        } else {
            winner = if card0 > card1 { 0 } else { 1 };
        }

        let (first, second) = if winner == 0 {
            (card0, card1)
        } else {
            (card1, card0)
        };

        decks[winner].push_back(first);
        decks[winner].push_back(second);
    }
    return winner;
}

fn part_2(deck1: &VecDeque<u32>, deck2: &VecDeque<u32>) -> u32 {
    let mut decks = [deck1.clone(), deck2.clone()];
    let winner = recursive_combat(&mut decks);

    let mut sum = 0;
    for (i, val) in decks[winner].iter().rev().enumerate() {
        sum += *val * (i as u32 + 1);
    }
    return sum;
}

fn main() {
    let input = include_bytes!("day022.input");
    let (deck1, deck2) = from_utf8(input)
        .unwrap()
        .strip_prefix("Player 1:")
        .unwrap()
        .trim_start()
        .split("Player 2:")
        .next_tuple()
        .unwrap();

    let deck1: VecDeque<u32> = deck1
        .trim()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let deck2: VecDeque<u32> = deck2
        .trim()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("Part 1: {}", part_1(&deck1, &deck2));
    println!("Part 2: {}", part_2(&deck1, &deck2));
}

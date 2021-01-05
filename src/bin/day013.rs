use std::str::from_utf8;
use itertools::Itertools;
use std::vec::Vec;
use num_integer::lcm;

fn part_1(timestamp: u64, bus_ids: &[u64]) -> u64 {
    let mut earliest_depart_time = u64::MAX;
    let mut bus_id = 0;
    for id in bus_ids {
        let mut time = 0;
        loop {
            time += *id;
            if time >= timestamp {
                break;
            }
        }
        if time < earliest_depart_time {
            bus_id = *id;
            earliest_depart_time = time;
        }
    }
    println!("Bus Id: {}, earliest depart time: {}", bus_id, earliest_depart_time);
    return (earliest_depart_time - timestamp) * bus_id;
}

fn part_2(bus_ids: &[(u64, u64)]) -> u64 {
    let mut timestamp = 0;
    let mut step = 1;

    for (diff, id) in bus_ids {
        while (timestamp + diff) % *id != 0 {
            timestamp += step;
        }
        step = lcm(step, *id);
    }

    return timestamp;
}

fn main() {
    let input = include_bytes!("day013.input");
    let (timestamp, ids) = from_utf8(input).unwrap().lines().next_tuple().unwrap();
    let timestamp: u64 = timestamp.parse().unwrap();
    let bus_ids: Vec<u64> = ids.split(',').filter_map(|s| s.parse::<u64>().ok()).collect();

    println!("Part 1, the ID of the earliest bus I can take to airport multiplied 
             by the number of minutes I need to wait for that bus: {}", part_1(timestamp, &bus_ids));

    let bus_ids: Vec<(u64, u64)> = ids.split(',').enumerate().filter_map(|(i, id)| {
        if id == "x" {
            return None;
        } else {
            return Some((i as _, id.parse::<u64>().unwrap()));
        }
    }).collect();
    println!("Part 2, timestamp: {}", part_2(&bus_ids));
}

use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let res_part_one = part_one(input.to_owned());
    println!("part one res: {:#?}", res_part_one);

    let res_part_two = part_two(input.to_owned());
    println!("part two res: {:#?}", res_part_two);
}

fn part_two(input: String) -> u32 {
    let res = input
        .trim()
        .split("\n")
        .map(|s| HashSet::<_>::from_iter(s.chars()))
        .collect::<Vec<_>>();

    res.chunks_exact(3)
        .map(|abc| -> HashSet<_> { &(&abc[0] & &abc[1]) & &abc[2] })
        .flat_map(|h| h.into_iter())
        .map(|c| char_to_value(c))
        .sum::<u32>()
}

fn part_one(input: String) -> u32 {
    input
        .trim()
        .split("\n")
        .map(|s| s.split_at(s.len() / 2))
        .flat_map(|(a, b)| {
            let seen = HashSet::<_>::from_iter(a.chars());

            HashSet::<_>::from_iter(b.chars().filter(|c| seen.contains(c))).into_iter()
        })
        .map(|c| char_to_value(c))
        .sum::<u32>()
}

fn char_to_value(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96, // 1..=26
        'A'..='Z' => (c as u32) - 38, // 27..=52
        _ => unreachable!(),
    }
}

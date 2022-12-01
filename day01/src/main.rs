use std::{collections::BinaryHeap, fs};

fn get_elf_calories() -> Vec<u32> {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let chunks = input.trim().split("\n\n");
    let elfs_calories = chunks
        .map(|c| {
            c.split("\n")
                .map(|s| s.parse::<u32>().unwrap())
                .fold(0, |a, b| a + b)
        })
        .collect::<Vec<_>>();

    elfs_calories
}

fn part_one() -> u32 {
    let elfs_calories = get_elf_calories();

    let max_calories = elfs_calories.iter().max().unwrap();

    *max_calories
}

fn part_two() -> u32 {
    let elf_calories = get_elf_calories();

    let mut heap = elf_calories.iter().collect::<BinaryHeap<_>>();
    let mut res = 0;
    for _ in 0..3 {
        if let Some(c) = heap.pop() {
            res += c;
        }
    }

    res
}

fn main() {
    let part_one_res = part_one();
    println!("part one res: {}", part_one_res);

    let part_two_res = part_two();
    println!("part two res: {}", part_two_res);
}

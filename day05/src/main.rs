use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let res_part_one = part_one(&input);
    println!("part one res: {:#?}", res_part_one);

    let res_part_two = part_two(&input);
    println!("part two res: {:#?}", res_part_two);
}

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Manifest {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

fn parse_input(input: &str) -> Manifest {
    let (header, moves) = input.trim().split_once("\n\n").unwrap();
    let nstacks = header
        .split("\n")
        .last()
        .unwrap()
        .trim()
        .split("  ")
        .count();

    let mut stacks = vec![VecDeque::new(); nstacks];

    for line in header.rsplit("\n").skip(1) {
        for (i, v) in line.chars().skip(1).step_by(4).enumerate() {
            if v == ' ' {
                continue;
            }

            stacks.get_mut(i).unwrap().push_front(v);
        }
    }

    let moves = moves
        .split("\n")
        .map(|line| {
            let vals = line.split(" ").skip(1).step_by(2).collect::<Vec<_>>();

            Move {
                quantity: vals[0].parse().unwrap(),
                from: vals[1].parse().unwrap(),
                to: vals[2].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    Manifest { stacks, moves }
}

fn part_one(input: &str) -> String {
    let mut manifest = parse_input(input);

    for mov in manifest.moves {
        for _ in 0..mov.quantity {
            let val = manifest.stacks[mov.from - 1].pop_front().expect(&format!(
                "we tried to pop from {} but it was empty",
                mov.from
            ));

            manifest.stacks[mov.to - 1].push_front(val);
        }
    }

    manifest
        .stacks
        .iter()
        .map(|stack| stack.front().unwrap_or(&' '))
        .collect()
}

fn part_two(input: &str) -> String {
    let mut manifest = parse_input(input);

    for mov in manifest.moves {
        let mut temp_stack = VecDeque::with_capacity(mov.quantity);

        for _ in 0..mov.quantity {
            let val = manifest.stacks[mov.from - 1].pop_front().expect(&format!(
                "we tried to pop from {} but it was empty",
                mov.from
            ));

            temp_stack.push_front(val);
        }

        for val in temp_stack {
            manifest.stacks[mov.to - 1].push_front(val);
        }
    }

    manifest
        .stacks
        .iter()
        .map(|stack| stack.front().unwrap_or(&' '))
        .collect()
}

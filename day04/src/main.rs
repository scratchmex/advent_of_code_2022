use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let res_part_one = part_one(&input);
    println!("part one res: {:#?}", res_part_one);

    let res_part_two = part_two(&input);
    println!("part two res: {:#?}", res_part_two);
}

fn parse_pairs(input: &String) -> Vec<((u32, u32), (u32, u32))> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut pairs = line.splitn(2, ",").map(|pair| {
                let mut range = pair.splitn(2, "-").map(|n| n.parse::<u32>().unwrap());

                (range.next().unwrap(), range.next().unwrap())
            });

            (pairs.next().unwrap(), pairs.next().unwrap())
        })
        .collect::<Vec<_>>()
}

fn part_one(input: &String) -> usize {
    parse_pairs(input)
        .iter()
        .filter(|((al, ar), (bl, br))| (al <= bl && br <= ar) || (bl <= al && ar <= br))
        .count()
}

fn part_two(input: &String) -> usize {
    parse_pairs(input)
        .iter()
        .filter(|((al, ar), (bl, br))| (bl <= ar) && (al <= br))
        .count()
}

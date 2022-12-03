use std::fs;

#[derive(Debug)]
enum HandChoice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum ResultChoice {
    Win,
    Lose,
    Draw,
}

fn calculate_points_from_handchoices(a: HandChoice, b: HandChoice) -> i32 {
    match (a, b) {
        // ordering
        // paper < scissors < rock < paper
        // 2     < 3        < 1    < 2
        // point per result
        // win lost draw
        // 6   0    3

        // (opponent, you)
        (HandChoice::Rock, HandChoice::Rock) => 1 + 3,
        (HandChoice::Rock, HandChoice::Paper) => 2 + 6,
        (HandChoice::Rock, HandChoice::Scissors) => 3 + 0,
        (HandChoice::Paper, HandChoice::Rock) => 1 + 0,
        (HandChoice::Paper, HandChoice::Paper) => 2 + 3,
        (HandChoice::Paper, HandChoice::Scissors) => 3 + 6,
        (HandChoice::Scissors, HandChoice::Rock) => 1 + 6,
        (HandChoice::Scissors, HandChoice::Paper) => 2 + 0,
        (HandChoice::Scissors, HandChoice::Scissors) => 3 + 3,
    }
}

fn part_one() -> i32 {
    let input = fs::read_to_string("src/input.txt").unwrap();

    input
        .trim()
        .split("\n")
        .map(|s| s.splitn(2, " ").collect::<Vec<_>>())
        .map(|v| {
            let a = match v[0] {
                "A" => HandChoice::Rock,
                "B" => HandChoice::Paper,
                "C" => HandChoice::Scissors,
                _ => unreachable!(),
            };

            let b = match v[1] {
                "X" => HandChoice::Rock,
                "Y" => HandChoice::Paper,
                "Z" => HandChoice::Scissors,
                _ => unreachable!(),
            };

            (a, b)
        })
        .map(|(a, b)| calculate_points_from_handchoices(a, b))
        .sum()
}

fn part_two() -> i32 {
    let input = fs::read_to_string("src/input.txt").unwrap();

    input
        .trim()
        .split("\n")
        .map(|s| s.splitn(2, " ").collect::<Vec<_>>())
        .map(|v| {
            let a = match v[0] {
                "A" => HandChoice::Rock,
                "B" => HandChoice::Paper,
                "C" => HandChoice::Scissors,
                _ => unreachable!(),
            };

            let b = match v[1] {
                "X" => ResultChoice::Lose,
                "Y" => ResultChoice::Draw,
                "Z" => ResultChoice::Win,
                _ => unreachable!(),
            };

            (a, b)
        })
        .map(|(a, b)| match (&a, b) {
            // (opponent_choice, result_expected)
            (HandChoice::Rock, ResultChoice::Win) => (a, HandChoice::Paper),
            (HandChoice::Rock, ResultChoice::Lose) => (a, HandChoice::Scissors),
            (HandChoice::Rock, ResultChoice::Draw) => (a, HandChoice::Rock),
            (HandChoice::Paper, ResultChoice::Win) => (a, HandChoice::Scissors),
            (HandChoice::Paper, ResultChoice::Lose) => (a, HandChoice::Rock),
            (HandChoice::Paper, ResultChoice::Draw) => (a, HandChoice::Paper),
            (HandChoice::Scissors, ResultChoice::Win) => (a, HandChoice::Rock),
            (HandChoice::Scissors, ResultChoice::Lose) => (a, HandChoice::Paper),
            (HandChoice::Scissors, ResultChoice::Draw) => (a, HandChoice::Scissors),
        })
        .map(|(a, b)| calculate_points_from_handchoices(a, b))
        .sum()
}

fn main() {
    let res_part_one = part_one();
    println!("part one res: {}", res_part_one);

    let res_part_two = part_two();
    println!("part two res: {}", res_part_two);
}

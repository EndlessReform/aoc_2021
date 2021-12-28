use std::collections::HashMap;
use std::error::Error;

enum Score {
    Corrupt(i64),
    Incomplete(i64),
}

impl Score {
    fn unpack(&self) -> i64 {
        match *self {
            Score::Corrupt(n) => n,
            Score::Incomplete(n) => n,
        }
    }
    fn is_corrupt(&self) -> bool {
        matches!(*self, Score::Corrupt(_))
    }
}

fn score(line: &str) -> Score {
    let chunk_pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let err_points_table = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let missing_points_table = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut stack: Vec<char> = Vec::new();
    for chr in line.chars() {
        if chunk_pairs.contains_key(&chr) {
            stack.push(chr);
        } else {
            // Assume no other chars for now
            match stack.pop() {
                Some(c) if chunk_pairs.get(&c) != Some(&chr) => {
                    return Score::Corrupt(*err_points_table.get(&chr).unwrap());
                }
                _ => continue,
            }
        }
    }

    let sum = stack
        .iter()
        .rev()
        .fold(0, |acc, c| (acc * 5) + missing_points_table.get(c).unwrap());
    Score::Incomplete(sum)
}

fn part1(s: &str) -> i64 {
    s.lines()
        .map(score)
        .filter(|s| s.is_corrupt())
        .fold(0, |acc, s| acc + s.unpack())
}

fn part2(s: &str) -> i64 {
    let mut all_scores: Vec<i64> = s
        .lines()
        .map(score)
        .filter(|s| !s.is_corrupt())
        .map(|s| s.unpack())
        .collect();

    all_scores.sort_unstable();
    all_scores[all_scores.len() / 2]
}

pub fn day10(s: &str) -> Result<(), Box<dyn Error>> {
    println!("Error score: {}", part1(s));
    println!("Middle score: {}", part2(s));
    Ok(())
}

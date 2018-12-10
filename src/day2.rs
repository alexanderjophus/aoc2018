use std::collections::HashSet;
use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("day2.txt");

    let p1: u32 = day2p1(input);
    let p2 = day2p2(input);

    println!("Day 2: p1: {}", p1);
    println!("Day 2: p2: {}", p2);
}

fn day2p1(input: &str) -> u32 {
    let mut twos = 0;
    let mut threes = 0;
    let mut freq = HashMap::new();

    for line in input.lines() {
        for c in line.chars() {
            *freq.entry(c).or_insert(0) += 1
        }

        if freq.values().any(|&f| f == 2) {
            twos += 1
        }

        if freq.values().any(|&f| f == 3) {
            threes += 1
        }
        freq.clear()
    }
    twos * threes
}

fn day2p2(input: &str) -> String {
    let mut seen = HashSet::new();

    for line in input.lines() {
        for (index, _) in line.chars().enumerate() {
            let new_line: String = line.chars().enumerate().map(|(j, c)| {
                if j == index {
                    '+'
                } else {
                    c
                }
            }).collect();
            if seen.contains(&new_line) {
                return new_line
            };
            seen.insert(new_line);
        }
    }
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(12, day2p1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"));
        assert_eq!(1, day2p1("aa\naaa"));
    }

    #[test]
    fn part2() {
        assert_eq!("a+", day2p2("ab\nac"));
        assert_eq!("a+c", day2p2("abc\nadc\nacb\neee\nfff"));
        assert_eq!("+oo", day2p2("too\ndoo\nabc\nefg\nhij"));
    }

}

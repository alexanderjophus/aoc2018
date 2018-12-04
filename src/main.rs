use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    day1();
    day2()
}

fn day1() {
    let input = include_str!("day1.txt");
        
    let p1: i32 = day1p1(input);
    let p2: i32 = day1p2(input);

    println!("Day 1: p1: {}", p1);
    println!("Day 1: p2: {}", p2);
}

fn day1p1(input: &str) -> i32 {
    input.lines()
        .map(|x| x.parse::<i32>().expect("can't parse value"))
        .sum()
}

fn day1p2(input: &str) -> i32 {
    let mut seen = HashSet::new();
	let mut sum = 0 as i32;
    seen.insert(0);
	input.lines()
        .cycle()
        .find_map(|x| {
            sum += x.parse::<i32>().expect("can't parse value");
            seen.replace(sum)
        }).unwrap()
}

fn day2() {
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
    fn test_day1() {
        assert_eq!(3, day1p1("+1\n-2\n+3\n+1"));
        assert_eq!(3, day1p1("+1\n+1\n+1"));
        assert_eq!(0, day1p1("+1\n+1\n-2"));
        assert_eq!(-6, day1p1("-1\n-2\n-3"));

        assert_eq!(0, day1p2("+1\n-1"));
        assert_eq!(10, day1p2("+3\n+3\n+4\n-2\n-4"));
        assert_eq!(5, day1p2("-6\n+3\n+8\n+5\n-6"));
        assert_eq!(14, day1p2("+7\n+7\n-2\n-7\n-4"));
    }

    #[test]
    fn test_day2() {
        assert_eq!(12, day2p1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"));
        assert_eq!(1, day2p1("aa\naaa"));

        assert_eq!("a+", day2p2("ab\nac"));
        assert_eq!("a+c", day2p2("abc\nadc\nacb\neee\nfff"));
        assert_eq!("+oo", day2p2("too\ndoo\nabc\nefg\nhij"));
    }
}

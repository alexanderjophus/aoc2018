use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("day1.txt");
        
    let p1 = day1p1(input);
    let p2 = day1p2(input);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(3, day1p1("+1\n-2\n+3\n+1"));
        assert_eq!(3, day1p1("+1\n+1\n+1"));
        assert_eq!(0, day1p1("+1\n+1\n-2"));
        assert_eq!(-6, day1p1("-1\n-2\n-3"));
    }

    #[test]
    fn part2() {
        assert_eq!(0, day1p2("+1\n-1"));
        assert_eq!(10, day1p2("+3\n+3\n+4\n-2\n-4"));
        assert_eq!(5, day1p2("-6\n+3\n+8\n+5\n-6"));
        assert_eq!(14, day1p2("+7\n+7\n-2\n-7\n-4"));
    }
}

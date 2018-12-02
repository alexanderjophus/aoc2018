use std::collections::HashSet;

fn main() {
    day1()
}

fn day1() {
    let input = include_str!("day1.txt");
        
    let p1: i32 = day1p1(input);
    let p2: i32 = day1p2(input);

    println!("p1: {}\np2: {}", p1, p2);
}

fn day1p1(input: &str) -> i32 {
    input.lines()
        .map(|x| x.parse::<i32>().expect("can't parse value"))
        .sum()
}

fn day1p2(input: &str) -> i32 {
    let mut seen = HashSet::new();
	let mut sum = 0 as i32;
	input.lines()
        .cycle()
        .find_map(|x| {
            sum += x.parse::<i32>().expect("can't parse value");
            seen.replace(sum)
        }).unwrap()
}

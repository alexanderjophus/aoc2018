extern crate regex;
use self::regex::Regex;

pub fn solve() {
     let input = include_str!("day3.txt");

    let p1: u32 = day3p1(input);
    // let p2 = day3p2(input);

    println!("Day 3: p1: {}", p1);
    // println!("Day 3: p2: {}", p2);   
}

#[derive(Debug)]
struct Fabric {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

fn day3p1(input: &str) -> u32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(?P<id>[\d]+) @ (?P<x>[\d]+),(?P<y>[\d]+): (?P<w>[\d]+)x(?P<h>[\d]+)").unwrap();
    }

    let fabrics = RE.captures_iter(input).filter_map(|cap| {
        let groups = (cap.name("id"), cap.name("x"), cap.name("y"), cap.name("w"), cap.name("h"));
        match groups {
            (Some(area), Some(x), Some(y), Some(w), Some(h)) => Some(Fabric {
                id: area.as_str().parse::<u32>().unwrap(),
                x: x.as_str().parse::<u32>().unwrap(),
                y: y.as_str().parse::<u32>().unwrap(),
                w: w.as_str().parse::<u32>().unwrap(),
                h: h.as_str().parse::<u32>().unwrap()
            }),
            _ => None,
        }
    });

    for _fabric in fabrics {
        
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(4, day3p1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }
}

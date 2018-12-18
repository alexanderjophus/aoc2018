extern crate regex;
extern crate arrayvec;
use std::collections::HashMap;
use std::collections::HashSet;
use self::regex::Regex;
use self::arrayvec::ArrayVec;

pub fn solve() {
     let input = include_str!("day3.txt");

    let p1 = day3p1(input);
    let p2 = day3p2(input);

    println!("Day 3: p1: {}", p1);
    println!("Day 3: p2: {}", p2);
}

#[derive(Debug)]
struct Fabric {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

fn day3p1(input: &str) -> usize {
    let mut claimed = HashMap::<(u32, u32), ArrayVec<[u32; 8]>>::new();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
    }

    for cap in RE.captures_iter(input) {
        let groups = (cap.name("id"), cap.name("x"), cap.name("y"), cap.name("w"), cap.name("h"));
        let fabric = match groups {
            (Some(id), Some(x), Some(y), Some(w), Some(h)) => Some(Fabric {
                id: id.as_str().parse::<u32>().unwrap(),
                x: x.as_str().parse::<u32>().unwrap(),
                y: y.as_str().parse::<u32>().unwrap(),
                w: w.as_str().parse::<u32>().unwrap(),
                h: h.as_str().parse::<u32>().unwrap()
            }),
            _ => None,
        }.unwrap();

        for i in fabric.x..fabric.x+fabric.w {
            for j in fabric.y..fabric.y+fabric.h {
                claimed.entry((i,j)).or_default().push(fabric.id);
            }
        }
    }

    claimed.values().filter(|v| v.len() > 1 ).count()
}

fn day3p2(input: &str) -> u32 {
    let mut claimed = HashMap::<(u32, u32), ArrayVec<[u32; 8]>>::new();
    let mut ids = HashSet::<u32>::new();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
    }

    for cap in RE.captures_iter(input) {
        let groups = (cap.name("id"), cap.name("x"), cap.name("y"), cap.name("w"), cap.name("h"));
        let fabric = match groups {
            (Some(id), Some(x), Some(y), Some(w), Some(h)) => Some(Fabric {
                id: id.as_str().parse::<u32>().unwrap(),
                x: x.as_str().parse::<u32>().unwrap(),
                y: y.as_str().parse::<u32>().unwrap(),
                w: w.as_str().parse::<u32>().unwrap(),
                h: h.as_str().parse::<u32>().unwrap()
            }),
            _ => None,
        }.unwrap();
        ids.insert(fabric.id);

        for i in fabric.x..fabric.x+fabric.w {
            for j in fabric.y..fabric.y+fabric.h {
                claimed.entry((i,j)).or_default().push(fabric.id);
            }
        }
    }

    // What is the ID of the only claim that doesn't overlap?
    for (_, claimed_ids) in claimed {
        // println!("{},{} has len {}", x, y, ids.len())
        if claimed_ids.len() > 1 {
            for id in claimed_ids {
                ids.remove(&id);
            }
        }
    };

    println!("id len {}", ids.len());

    let mut ret = 0;
    for id in ids {
        ret = id
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1() {
        assert_eq!(4, day3p1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }

    #[test]
    fn part2() {
        assert_eq!(3, day3p2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| day3p1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| day3p2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }

}

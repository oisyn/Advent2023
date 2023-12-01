use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FIND: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const FIND_REV: &[&str] = &[
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn find_digit(s: &str, matches: &'static [&str]) -> i32 {
    let mut p: Option<(_, _)> = None;

    for (idx, &m) in matches.iter().enumerate() {
        if let Some(pos) = s.find(m) {
            match p {
                Some(old) if old.1 < pos => {}
                _ => p = Some((idx, pos)),
            };
        }
    }

    let max = p.map_or(s.len(), |p| p.1);
    let d = s.as_bytes()[0..max]
        .iter()
        .find(|&&c| c >= b'0' && c <= b'9');

    if let Some(c) = d {
        (c - b'0') as i32
    } else {
        (p.unwrap().0 + 1) as i32
    }
}

fn find_first_digit(s: &str) -> i32 {
    find_digit(s, FIND)
}

fn find_last_digit(s: &str) -> i32 {
    find_digit(&s.chars().rev().collect::<String>(), FIND_REV)
}

fn main() -> Result<()> {
    let f = File::open("day1/data/input.txt")?;
    let f = BufReader::new(f);

    let mut total1 = 0;
    let mut total2 = 0;

    for l in f.lines() {
        let l = l?;
        let b = l.as_bytes();

        let i0 = b.iter().find(|&&c| c >= b'0' && c <= b'9');
        let i1 = b.iter().rev().find(|&&c| c >= b'0' && c <= b'9');
        if let (Some(i0), Some(i1)) = (i0, i1) {
            let value = (i0 - b'0') as i32 * 10 + (i1 - b'0') as i32;
            total1 += value;
        }

        let i0 = find_first_digit(&l);
        let i1 = find_last_digit(&l);

        let value = i0 * 10 + i1;
        total2 += value;
    }

    println!("{total1} {total2}");
    Ok(())
}

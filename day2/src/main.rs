#![allow(dead_code)]

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use util::*;

enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl FromParser for Color {
    fn parse_from<'a>(parser: &mut Parser<'a>) -> Option<Self> {
        match parser.peek_char() {
            Some(b'r') => {
                parser.skip("red".len());
                Some(Self::Red)
            }
            Some(b'g') => {
                parser.skip("green".len());
                Some(Self::Green)
            }
            Some(b'b') => {
                parser.skip("blue".len());
                Some(Self::Blue)
            }
            _ => None,
        }
    }
}

fn main() -> Result<()> {
    let f = File::open("day2/data/input.txt")?;
    let f = BufReader::new(f);

    const MAX_CUBES: [i32; 3] = [12, 13, 14];

    let mut total1 = 0;
    let mut total2 = 0;

    for l in f.lines() {
        let l = l?;
        let mut p = Parser::new(&l);

        p.skip("Game ".len());
        let game: i32 = p.parse().unwrap();
        p.skip(": ".len());

        let mut max_cubes = [0, 0, 0];
        let mut ok1 = true;
        while !p.at_end() {
            let num: i32 = p.parse().unwrap();
            let color: Color = p.skip(1).parse().unwrap();
            let color = color as usize;

            ok1 &= num <= MAX_CUBES[color];
            max_cubes[color] = max_cubes[color].max(num);

            p.skip(2);
        }

        if ok1 {
            total1 += game;
        }
        total2 += max_cubes.iter().product::<i32>();
    }

    println!("{total1} {total2}");

    Ok(())
}

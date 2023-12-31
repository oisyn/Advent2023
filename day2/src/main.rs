use anyhow::Result;
use util::*;

enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl FromParser<'_> for Color {
    fn parse_from(parser: &mut Parser) -> Option<Self> {
        match parser.peek_char() {
            Some(b'r') => {
                parser.expect("red");
                Some(Self::Red)
            }
            Some(b'g') => {
                parser.expect("green");
                Some(Self::Green)
            }
            Some(b'b') => {
                parser.expect("blue");
                Some(Self::Blue)
            }
            _ => None,
        }
    }
}

fn main() -> Result<()> {
    let input = open_input("day2")?;

    const MAX_CUBES: [i32; 3] = [12, 13, 14];

    let mut total1 = 0;
    let mut total2 = 0;

    for l in input.lines() {
        let mut p = Parser::new(l);

        p.expect("Game ");
        let game: i32 = p.parse().unwrap();
        p.expect(": ");
        let mut max_cubes = [0, 0, 0];
        let mut ok1 = true;
        while !p.at_end() {
            let num: i32 = p.parse().unwrap();
            let color: Color = p.expect(" ").parse().unwrap();
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

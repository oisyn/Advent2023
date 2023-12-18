use anyhow::Result;
use util::*;

#[derive(Copy, Clone, Debug)]
struct Vec2(i32, i32);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Border {
    Vertical(i32, i32),
    SameSpan(i32, i32, i32),
    FlipSpan(i32, i32, i32),
}

impl Border {
    pub fn x_min(self) -> i32 {
        match self {
            Self::Vertical(_, x) => x,
            Self::SameSpan(_, x, _) => x,
            Self::FlipSpan(_, x, _) => x,
        }
    }

    pub fn y(self) -> i32 {
        match self {
            Self::Vertical(y, _) => y,
            Self::SameSpan(y, _, _) => y,
            Self::FlipSpan(y, _, _) => y,
        }
    }
}

impl PartialOrd for Border {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.y(), self.x_min()).partial_cmp(&(other.y(), other.x_min()))
    }
}

impl Ord for Border {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y(), self.x_min()).cmp(&(other.y(), other.x_min()))
    }
}

fn main() -> Result<()> {
    let input = open_input("day18")?;

    let mut coords1 = Vec::with_capacity(1000);
    let mut coords2 = Vec::with_capacity(1000);
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);

    for l in input.lines() {
        let mut p = Parser::new(l);
        let c = p.take_char().unwrap();
        let n = p.expect(" ").parse::<i32>().unwrap();

        match c {
            b'U' => pos1.1 -= n,
            b'D' => pos1.1 += n,
            b'L' => pos1.0 -= n,
            _ => pos1.0 += n,
        }

        coords1.push(pos1);

        let n = i32::from_str_radix(p.expect(" (#").take(5), 16).unwrap();
        let c = p.take_char().unwrap();

        match c {
            b'0' => pos2.0 += n,
            b'1' => pos2.1 += n,
            b'2' => pos2.0 -= n,
            _ => pos2.1 -= n,
        }

        coords2.push(pos2);
    }

    let total1 = area(coords1);
    let total2 = area(coords2);

    println!("{total1} {total2}");

    Ok(())
}

fn area(mut coords: Vec<(i32, i32)>) -> u64 {
    coords.push(coords[0]);
    coords.push(coords[1]);
    coords.push(coords[2]);
    let mut borders = Vec::with_capacity(1000);
    for c in coords.windows(4).filter(|&c| c[0].1 != c[1].1) {
        assert!(c[0].0 == c[1].0 && c[2].0 == c[3].0);

        let ld = c[1].1 > c[0].1;
        let rd = c[3].1 > c[2].1;

        let min = c[1].0.min(c[2].0);
        let max = c[1].0.max(c[2].0);

        for y in match ld {
            true => c[0].1 + 1..c[1].1,
            false => c[1].1 + 1..c[0].1,
        } {
            borders.push(Border::Vertical(y, c[0].0));
        }

        if ld != rd {
            borders.push(Border::SameSpan(c[1].1, min, max));
        } else {
            borders.push(Border::FlipSpan(c[1].1, min, max));
        }
    }
    borders.sort();

    let mut area = 0;
    let mut iter = borders.into_iter();
    loop {
        match iter.next() {
            None => break,
            Some(Border::SameSpan(_, x0, x1)) => {
                area += (x1 - x0 + 1) as u64;
            }
            Some(Border::Vertical(_, x0) | Border::FlipSpan(_, x0, _)) => loop {
                match iter.next().unwrap() {
                    Border::Vertical(_, x1) => {
                        area += (x1 - x0 + 1) as u64;
                        break;
                    }
                    Border::SameSpan(..) => {}
                    Border::FlipSpan(_, _, x1) => {
                        area += (x1 - x0 + 1) as u64;
                        break;
                    }
                }
            },
        }
    }

    area
}

#![allow(dead_code)]

use anyhow::Result;
use util::*;

const UP_LEFT: u8 = b'J';
const UP_RIGHT: u8 = b'L';
const DOWN_LEFT: u8 = b'7';
const DOWN_RIGHT: u8 = b'F';
const VERTICAL: u8 = b'|';
const HORIZONTAL: u8 = b'-';

const FLAG_LEFT: u8 = 1;
const FLAG_RIGHT: u8 = 2;
const FLAG_UP: u8 = 4;
const FLAG_DOWN: u8 = 8;

const fn gen_table() -> [u8; 256] {
    let mut t = [0; 256];

    t[UP_LEFT as usize] = FLAG_UP | FLAG_LEFT;
    t[UP_RIGHT as usize] = FLAG_UP | FLAG_RIGHT;
    t[DOWN_LEFT as usize] = FLAG_DOWN | FLAG_LEFT;
    t[DOWN_RIGHT as usize] = FLAG_DOWN | FLAG_RIGHT;
    t[VERTICAL as usize] = FLAG_UP | FLAG_DOWN;
    t[HORIZONTAL as usize] = FLAG_LEFT | FLAG_RIGHT;

    return t;
}

const TABLE: [u8; 256] = gen_table();

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

impl Direction {
    pub fn from_index(index: u8) -> Self {
        [Self::Left, Self::Right, Self::Up, Self::Down][index as usize]
    }

    pub fn invert(self) -> Self {
        Self::from_index(self as u8 ^ 1)
    }

    pub fn flag(self) -> u8 {
        1 << self as u8
    }
}

trait PipeSection {
    fn has_left(self) -> bool;
    fn has_right(self) -> bool;
    fn has_up(self) -> bool;
    fn has_down(self) -> bool;
    fn flags(self) -> u8;
    fn next_dir(self, dir: Direction) -> Direction;
}

impl PipeSection for u8 {
    fn has_left(self) -> bool {
        TABLE[self as usize] & FLAG_LEFT != 0
    }

    fn has_right(self) -> bool {
        TABLE[self as usize] & FLAG_RIGHT != 0
    }

    fn has_up(self) -> bool {
        TABLE[self as usize] & FLAG_UP != 0
    }

    fn has_down(self) -> bool {
        TABLE[self as usize] & FLAG_DOWN != 0
    }

    fn flags(self) -> u8 {
        TABLE[self as usize]
    }

    fn next_dir(self, dir: Direction) -> Direction {
        let orig = dir as u8 ^ 1;
        for i in 0..4 {
            let m = 1_u8 << i;
            if orig != i && TABLE[self as usize] & m != 0 {
                return Direction::from_index(i);
            }
        }
        unreachable!();
    }
}

struct Field {
    pub data: Vec<u8>,
    pub width: isize,
    pub height: isize,
    pub stride: isize,
    pub start_offset: isize,
}

impl Field {
    pub fn new(data: &[u8]) -> Self {
        let data = data.to_owned();
        let width = data.iter().position(|&b| b == b'\r' || b == b'\n').unwrap() as isize;
        let stride = width + 1 + (data[width as usize + 1] == b'\n') as isize;
        let height = (data.len() + 2) as isize / stride;

        let start_offset = data.iter().position(|&b| b == b'S').unwrap() as isize;

        let mut s = Self {
            data,
            width,
            height,
            stride,
            start_offset,
        };
        s.replace_start();
        s
    }

    pub fn replace_start(&mut self) {
        let s = self.start_offset;
        let mut flags = 0;

        if self[s - 1].has_right() {
            flags |= FLAG_LEFT;
        }
        if self[s + 1].has_left() {
            flags |= FLAG_RIGHT;
        }
        if self[s - self.stride].has_down() {
            flags |= FLAG_UP;
        }
        if self[s + self.stride].has_up() {
            flags |= FLAG_DOWN;
        }

        for &b in "FJL7|-".as_bytes() {
            if b.flags() == flags {
                self.data[s as usize] = b;
                return;
            }
        }

        unreachable!();
    }

    pub fn next(&self, (dir, offset): (Direction, isize)) -> (Direction, isize) {
        let next_dir = self[offset].next_dir(dir);
        (
            next_dir,
            offset + [-1, 1, -self.stride, self.stride][next_dir as usize],
        )
    }

    pub fn iter_path(&self) -> PathIter {
        PathIter {
            field: self,
            loc: (Direction::Left, -1),
        }
    }

    pub fn get(&self, x: isize, y: isize) -> u8 {
        self[y * self.stride + x]
    }
}

impl std::ops::Index<isize> for Field {
    type Output = u8;
    fn index(&self, index: isize) -> &Self::Output {
        const EMPTY: &u8 = &b'.';

        if index < 0 || index >= self.data.len() as isize {
            EMPTY
        } else {
            &self.data[index as usize]
        }
    }
}

struct PathIter<'a> {
    field: &'a Field,
    loc: (Direction, isize),
}

impl<'a> Iterator for PathIter<'a> {
    type Item = (Direction, isize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.loc.1 == -1 {
            self.loc = (Direction::Up, self.field.start_offset);
            Some(self.loc)
        } else {
            let n = self.field.next(self.loc);
            if n.1 == self.field.start_offset {
                None
            } else {
                self.loc = n;
                Some(n)
            }
        }
    }
}

enum Border {
    /// |
    Vertical(isize),

    /// F..7 or L..J
    SameSpan(isize, isize),

    /// F..J or L..7
    FlipSpan(isize, isize),
}

fn next_border(field: &Field, iter: &mut impl Iterator<Item = isize>) -> Option<Border> {
    let o0 = iter.next()?;
    let c0 = field[o0];
    if c0 == VERTICAL {
        return Some(Border::Vertical(o0));
    }

    let o1 = iter.next().unwrap();
    let c1 = field[o1];
    if c0 == UP_RIGHT && c1 == UP_LEFT || c0 == DOWN_RIGHT && c1 == DOWN_LEFT {
        Some(Border::SameSpan(o0, o1))
    } else {
        Some(Border::FlipSpan(o0, o1))
    }
}

fn main() -> Result<()> {
    let input = open_input("day10")?;
    let field = Field::new(input.bytes());

    let mut vert_offsets = Vec::with_capacity(1000);

    let mut count = 0;
    for (_, o) in field.iter_path() {
        count += 1;
        if field[o] != HORIZONTAL {
            vert_offsets.push(o);
        }
    }

    vert_offsets.sort();
    let mut area = 0;
    let mut iter = vert_offsets.into_iter();
    loop {
        match next_border(&field, &mut iter) {
            None => break,
            Some(Border::SameSpan(o0, o1)) => {
                area += o1 - o0 + 1;
            }
            Some(Border::Vertical(o0) | Border::FlipSpan(o0, _)) => loop {
                match next_border(&field, &mut iter).unwrap() {
                    Border::Vertical(o1) => {
                        area += o1 - o0 + 1;
                        break;
                    }
                    Border::SameSpan(_, _) => {}
                    Border::FlipSpan(_, o1) => {
                        area += o1 - o0 + 1;
                        break;
                    }
                }
            },
        }
    }

    let total1 = count / 2;
    let total2 = area - count;

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

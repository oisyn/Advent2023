use std::collections::VecDeque;

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day4")?;

    let mut iter = input.bytes().iter();
    let winning_start = iter.position(|&c| c == b':').unwrap() + 2;
    let winning_end = winning_start + iter.position(|&c| c == b'|').unwrap() - 2;
    let own_start = winning_end + 3;
    let own_end = winning_end + 2 + iter.position(|&c| c == b'\r' || c == b'\n').unwrap();

    let mut total1 = 0;
    let mut total2 = 0;

    let mut copies = VecDeque::with_capacity(1000);
    copies.push_back(1);

    for l in input.lines() {
        let mut winning = 0_u128;
        for n in to_str(&l.as_bytes()[winning_start..winning_end]).split_ascii_whitespace() {
            let number: u32 = n.parse().unwrap();
            winning |= 1_u128 << number;
        }

        let mut num_winning = 0;
        for n in to_str(&l.as_bytes()[own_start..own_end]).split_ascii_whitespace() {
            let number: u32 = n.parse().unwrap();
            if winning & (1_u128 << number) != 0 {
                num_winning += 1;
            }
        }

        let num_copies = copies.pop_front().unwrap();
        total2 += num_copies;

        let len = match num_winning {
            0 => 1,
            _ => num_winning,
        };

        copies.resize(copies.len().max(len as usize), 1);

        if num_winning > 0 {
            total1 += 1 << (num_winning - 1);
            for n in copies.iter_mut().take(num_winning) {
                *n += num_copies;
            }
        }
    }

    println!("{total1} {total2}");

    Ok(())
}

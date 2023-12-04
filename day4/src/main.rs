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

    let mut copies: Vec<(i32, i32)> = Vec::with_capacity(1000);

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

        let mut num_copies = 1;
        copies.retain_mut(|(c, n)| {
            num_copies += *c;
            *n -= 1;
            *n > 0
        });
        total2 += num_copies;

        if num_winning > 0 {
            total1 += 1 << (num_winning - 1);
            copies.push((num_copies, num_winning));
        }
    }

    println!("{total1} {total2}");

    Ok(())
}

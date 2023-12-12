use std::collections::HashMap;

use anyhow::Result;
use util::*;

const MAX_LENGTHS: usize = 64;

fn calc<'a, 'b>(
    mut broken: u128,
    mut working: u128,
    mut len: u32,
    lengths: &'a [u32],
    cache: &'b mut HashMap<(u128, u128, u32, &'a [u32]), u64>,
) -> u64 {
    if let Some(&r) = cache.get(&(broken, working, len, lengths)) {
        return r;
    }

    let r = 'r: {
        if lengths.is_empty() {
            break 'r (broken == 0) as u64;
        }

        if len + 1 < lengths.iter().sum::<u32>() + lengths.len() as u32 {
            break 'r 0;
        }

        let l = lengths[0];
        let m = (1_u128 << l) - 1;

        while len >= l && working & 1 == 1 {
            broken >>= 1;
            working >>= 1;
            len -= 1;
        }

        if len < l {
            break 'r 0;
        }

        let mut r = if working & m == 0 && broken & (m + 1) == 0 {
            calc(
                broken >> l + 1,
                working >> l + 1,
                len.saturating_sub(l + 1),
                &lengths[1..],
                cache,
            )
        } else {
            0
        };

        if broken & 1 != 1 {
            r += calc(broken >> 1, working >> 1, len - 1, lengths, cache);
        }

        r
    };
    cache.insert((broken, working, len, lengths), r);
    r
}

fn main() -> Result<()> {
    let input = open_input("day12")?;

    let mut lengths = Vec::with_capacity(MAX_LENGTHS);
    let mut total1 = 0;
    let mut total2 = 0;
    for l in input.lines() {
        let mut p = Parser::new(l);
        let mut broken = 0;
        let mut working = 0;

        let mut m = 1;
        let mut line_len = 0;
        loop {
            match p.take_char().unwrap() {
                b' ' => break,
                b'.' => working |= m,
                b'#' => broken |= m,
                b'?' => {}
                _ => unreachable!(),
            }
            m <<= 1;
            line_len += 1;
        }

        lengths.clear();
        while !p.at_end() {
            lengths.push(p.parse::<u32>().unwrap());
            p.skip(1);
        }

        if lengths.len() > MAX_LENGTHS {
            panic!();
        }

        total1 += calc(broken, working, line_len, &lengths, &mut HashMap::new());
        let mut broken2 = broken;
        let mut working2 = working;
        let lengths = lengths.repeat(5);

        for i in 1..5 {
            broken2 |= broken << i * (line_len + 1);
            working2 |= working << i * (line_len + 1);
        }

        total2 += calc(
            broken2,
            working2,
            line_len * 5 + 4,
            &lengths,
            &mut HashMap::new(),
        );
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

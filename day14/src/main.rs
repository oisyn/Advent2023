#![allow(dead_code)]

use std::{arch::x86_64::_pext_u64, collections::HashMap};

use anyhow::Result;
use util::*;

fn as_bytes_mut<T>(data: &mut [T]) -> &mut [u8] {
    unsafe {
        std::slice::from_raw_parts_mut(
            data.as_mut_ptr() as *mut u8,
            data.len() * std::mem::size_of::<T>(),
        )
    }
}

fn transpose(data: &mut [u128]) {
    fn read8x8(source: &[u8]) -> u64 {
        let mut s = 0_u64;
        for i in 0..8 {
            s |= (source[i * 16] as u64) << (i * 8);
        }
        s
    }

    fn write8x8(s: u64, dest: &mut [u8]) {
        const SOURCE_PATTERN: u64 = 0x01010101_01010101;

        for i in 0..8 {
            dest[i * 16] = unsafe { _pext_u64(s, SOURCE_PATTERN << i) as u8 };
        }
    }

    let data = as_bytes_mut(data);

    for y in 0..16 {
        let s = read8x8(&data[y * 128 + y..]);
        write8x8(s, &mut data[y * 128 + y..]);

        for x in y + 1..16 {
            let s0 = read8x8(&data[y * 128 + x..]);
            let s1 = read8x8(&data[x * 128 + y..]);
            write8x8(s0, &mut data[x * 128 + y..]);
            write8x8(s1, &mut data[y * 128 + x..]);
        }
    }
}

fn flip(data: &mut [u128], h: usize) {
    for y in 0..h / 2 {
        (data[y], data[h - y - 1]) = (data[h - y - 1], data[y]);
    }
}

fn rotate_cw(dest: &mut [u128], h: usize) {
    flip(dest, h);
    transpose(dest);
}

fn fall_north(rocks: &mut [u128], fixed: &[u128], occupied: &mut [u128], h: usize) {
    for mut y in 0..h {
        occupied[y] = fixed[y];

        let mut cur = rocks[y];
        rocks[y] = 0;
        while y > 0 && cur != 0 {
            let immovable = cur & occupied[y - 1];
            occupied[y] |= immovable;
            rocks[y] |= immovable;
            y -= 1;
            cur ^= immovable;
        }

        if cur > 0 {
            occupied[0] |= cur;
            rocks[0] |= cur;
        }
    }
}

fn weight(rocks: &[u128], h: usize) -> u32 {
    let mut total = 0;

    for y in 0..h {
        total += (h - y) as u32 * rocks[y].count_ones();
    }

    total
}

fn show(rocks: &[u128], fixed: &[u128], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            if rocks[y] & (1 << x) != 0 {
                print!("O");
            } else if fixed[y] & (1 << x) != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() -> Result<()> {
    let input = open_input("day14")?;
    let data = input.bytes();

    let width = data.iter().position(|&b| b == b'\r' || b == b'\n').unwrap();
    assert!(width < 128);
    let stride = width + 1 + (data[width + 1] == b'\n') as usize;
    let height = (data.len() + 2) / stride;
    let field = FieldView::new(data, width, stride, height);
    let mut rocks = vec![0_u128; 128];
    let mut fixed_n = vec![0_u128; 128];
    let mut occupied = vec![0_u128; 128];

    let mut total1 = 0;
    let mut total2 = 0;

    for (y, row) in field.rows().enumerate() {
        let mut r: u128 = 0_u128;
        let mut f = 0_u128;
        for x in 0..width {
            match row[x] {
                b'O' => r |= 1_u128 << x,
                b'#' => f |= 1_u128 << x,
                _ => {}
            }
        }
        rocks[y] = r;
        fixed_n[y] = f;
    }

    let mut fixed_w = fixed_n.clone();
    rotate_cw(&mut fixed_w, height);

    let mut fixed_s = fixed_w.clone();
    rotate_cw(&mut fixed_s, width);

    let mut fixed_e = fixed_s.clone();
    rotate_cw(&mut fixed_e, height);

    let mut history: HashMap<_, _> = HashMap::new();

    for cycle in 0..1_000_000_000 {
        let (prev, _) = *history
            .entry(rocks.clone())
            .or_insert_with(|| (cycle, weight(&rocks, height)));
        if prev < cycle {
            let len = cycle - prev;
            let d = (1_000_000_000 - prev) % len + prev;
            total2 = history
                .values()
                .find_map(|&(c, w)| (c == d).then_some(w))
                .unwrap();
            break;
        }

        fall_north(&mut rocks, &fixed_n, &mut occupied, height);
        if cycle == 0 {
            total1 = weight(&rocks, height);
        }

        rotate_cw(&mut rocks, height);
        fall_north(&mut rocks, &fixed_w, &mut occupied, width);

        rotate_cw(&mut rocks, width);
        fall_north(&mut rocks, &fixed_s, &mut occupied, height);

        rotate_cw(&mut rocks, height);
        fall_north(&mut rocks, &fixed_e, &mut occupied, width);

        rotate_cw(&mut rocks, width);
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

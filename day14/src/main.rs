#![allow(dead_code)]

use std::collections::HashMap;

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day14")?;
    let data = input.bytes();

    let width = data.iter().position(|&b| b == b'\r' || b == b'\n').unwrap();
    let stride = width + 1 + (data[width + 1] == b'\n') as usize;
    let height = (data.len() + 2) / stride;
    let field = FieldView::new(data, width, stride, height);
    let mut rocks = Vec::with_capacity(10000);
    let mut fences = Vec::with_capacity(10000);

    let mut total1 = 0;
    let mut total2 = 0;
    let mut weight = 0;

    let mut field_w = vec![0_u32; width * height];
    for (y, row) in field.rows().enumerate() {
        let mut fence_idx = fences.len() as u32;
        let o = (y * width) as u32;
        fences.push((o.wrapping_sub(1), [0, 0, 0, 0]));

        for x in 0..width {
            let o = o + x as u32;
            match row[x] {
                b'O' => rocks.push(o),
                b'#' => {
                    fence_idx = fences.len() as u32;
                    fences.push((o, [0, 0, 0, 0]));
                }
                _ => {}
            }
            field_w[o as usize] = fence_idx;
        }
    }

    let mut field_n = vec![0_u32; width * height];
    for x in 0..width {
        let mut fence_idx = fences.len() as u32;
        let o = x as u32;
        fences.push((o.wrapping_sub(width as u32), [0, 0, 0, 0]));
        for (y, &b) in field.col(x).into_iter().enumerate() {
            let o = (y * width) as u32 + o;
            if b == b'#' {
                fence_idx = field_w[o as usize];
            }
            field_n[o as usize] = fence_idx;
        }
    }

    let mut field_e = vec![0_u32; width * height];
    for y in 0..height {
        let mut fence_idx = fences.len() as u32;
        let o = (y * width) as u32;
        fences.push((o + width as u32, [0, 0, 0, 0]));
        for (x, &b) in field.row(y).into_iter().enumerate().rev() {
            let o = o + x as u32;
            if b == b'#' {
                fence_idx = field_w[o as usize];
            }
            field_e[o as usize] = fence_idx;
        }
    }

    let mut field_s = vec![0_u32; width * height];
    for x in 0..width {
        let mut fence_idx = fences.len() as u32;
        let o = x as u32;
        fences.push((o + (width * height) as u32, [0, 0, 0, 0]));
        for (y, &b) in field.col(x).into_iter().enumerate().rev() {
            let o = (y * width) as u32 + o;
            if b == b'#' {
                fence_idx = field_w[o as usize];
            }
            field_s[o as usize] = fence_idx;
        }
    }

    for &r in &rocks {
        let mut o;
        o = field_n[r as usize];
        fences[o as usize].1[0] += 1;
        o = fences[o as usize]
            .0
            .wrapping_add(fences[o as usize].1[0] * width as u32);
        total1 += height as u32 - (o / width as u32);
    }

    let mut history = HashMap::new();

    for cycle in 0..1_000_000_000 {
        rocks.sort();
        let (prev, _) = *history.entry(rocks.clone()).or_insert((cycle, weight));
        if prev < cycle {
            let len = cycle - prev;
            let d = (1_000_000_000 - prev) % len + prev;
            total2 = history
                .values()
                .find_map(|&(c, w)| (c == d).then_some(w))
                .unwrap();
            break;
        }

        for f in &mut fences {
            f.1 = [0, 0, 0, 0];
        }

        weight = 0;

        for r in &mut rocks {
            let mut o;
            o = field_n[*r as usize];
            fences[o as usize].1[0] += 1;
            o = fences[o as usize]
                .0
                .wrapping_add(fences[o as usize].1[0] * width as u32);

            o = field_w[o as usize];
            fences[o as usize].1[1] += 1;
            o = fences[o as usize].0.wrapping_add(fences[o as usize].1[1]);

            o = field_s[o as usize];
            fences[o as usize].1[2] += 1;
            o = fences[o as usize]
                .0
                .wrapping_sub(fences[o as usize].1[2] * width as u32);

            o = field_e[o as usize];
            fences[o as usize].1[3] += 1;
            o = fences[o as usize].0.wrapping_sub(fences[o as usize].1[3]);

            *r = o;
            weight += height as u32 - (o / width as u32);
        }
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

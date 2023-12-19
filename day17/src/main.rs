use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day17")?;
    let data = input.bytes();

    let width = data.iter().position(|&b| is_nl(b)).unwrap();
    let stride = width + 1 + (data[width + 1] == b'\n') as usize;
    let height = (data.len() + 2) / stride;

    const HORIZONTAL: u8 = 0;
    const VERTICAL: u8 = 1;

    let mut min_heat = vec![[u32::MAX, u32::MAX]; height * stride];
    let mut queue = BinaryHeap::with_capacity(10000);
    min_heat[0] = [0, 0];
    let valid_range = 0..data.len() as i32;
    let end_pos = ((height - 1) * stride + width - 1) as i32;

    let mut total1 = 0;
    let mut total2 = 0;

    for (r, min_walk, max_walk) in [(&mut total1, 0, 3), (&mut total2, 4, 10)] {
        min_heat.fill([u32::MAX, u32::MAX]);
        queue.clear();

        queue.push(Reverse((0, 0, HORIZONTAL)));
        queue.push(Reverse((0, 0, VERTICAL)));

        *r = loop {
            let (w, pos, dir) = queue.pop().unwrap().0;
            if w > min_heat[pos as usize][dir as usize] {
                continue;
            }

            if pos == end_pos {
                break w;
            }

            let scale = [1, stride as i32][dir as usize];
            let new_dir = 1 - dir;

            let mut l0 = w;
            let mut l1 = w;
            let mut d0 = true;
            let mut d1 = true;
            for step in 1..=max_walk {
                let new = pos + step * scale;
                if d0 && valid_range.contains(&new) && !is_nl(data[new as usize]) {
                    l0 += (data[new as usize] - b'0') as u32;
                    if step >= min_walk && min_heat[new as usize][new_dir as usize] > l0 {
                        min_heat[new as usize][new_dir as usize] = l0;
                        queue.push(Reverse((l0, new, new_dir)));
                    }
                } else {
                    d0 = false;
                }
                let new = pos - step * scale;
                if d1 && valid_range.contains(&new) && !is_nl(data[new as usize]) {
                    l1 += (data[new as usize] - b'0') as u32;
                    if step >= min_walk && min_heat[new as usize][new_dir as usize] > l1 {
                        min_heat[new as usize][new_dir as usize] = l1;
                        queue.push(Reverse((l1, new, new_dir)));
                    }
                } else {
                    d1 = false;
                }

                if !d0 && !d1 {
                    break;
                }
            }
        };
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

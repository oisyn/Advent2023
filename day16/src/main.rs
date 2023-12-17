use std::collections::HashSet;

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day16")?;
    let data = input.bytes();

    let width = data.iter().position(|&b| b == b'\r' || b == b'\n').unwrap();
    let stride = width + 1 + (data[width + 1] == b'\n') as usize;
    let height = (data.len() + 2) / stride;

    let mut energized = HashSet::with_capacity(20000);
    let mut splitter_done = HashSet::with_capacity(1000);
    let mut queue = Vec::with_capacity(100);
    let mut total1 = 0;
    let mut total2 = 0;

    let valid_range = 0..data.len() as i32;
    let dir_offsets = [1, stride as i32, -1, -(stride as i32)];

    for (pos, dir) in (0..width)
        .flat_map(|x| [(x, 1), (x + (height - 1) * stride, 3)])
        .chain((0..height).flat_map(|y| [(y * stride, 0), (y * stride + width - 1, 2)]))
    {
        energized.clear();
        queue.clear();
        splitter_done.clear();

        queue.push((pos as i32, dir));

        while !queue.is_empty() {
            let (mut pos, mut dir) = queue.pop().unwrap();

            loop {
                if !valid_range.contains(&pos)
                    || data[pos as usize] == b'\r'
                    || data[pos as usize] == b'\n'
                {
                    break;
                }

                energized.insert(pos);

                match data[pos as usize] {
                    b'.' => {}
                    b'\\' => dir ^= 1,
                    b'/' => dir ^= 3,
                    b'|' if (dir & 1 == 0) => {
                        if !splitter_done.insert(pos) {
                            break;
                        }
                        queue.push((pos - stride as i32, 3));
                        dir = 1;
                    }
                    b'|' => {
                        splitter_done.insert(pos);
                    }
                    b'-' if (dir & 1 != 0) => {
                        if !splitter_done.insert(pos) {
                            break;
                        }
                        queue.push((pos - 1, 2));
                        dir = 0;
                    }
                    b'-' => {
                        splitter_done.insert(pos);
                    }
                    _ => unreachable!(),
                }

                pos += dir_offsets[dir as usize];
            }
        }

        if pos == 0 && dir == 0 {
            total1 = energized.len();
        }
        total2 = total2.max(energized.len());
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

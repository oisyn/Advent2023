use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day21")?;
    let mut data = input.bytes().to_owned();

    let width = data.iter().position(|&b| is_nl(b)).unwrap();
    let stride = width + 1 + (data[width + 1] == b'\n') as usize;
    let height = (data.len() + 2) / stride;
    assert!(width == height);
    assert!(width & 1 == 1);

    let start = data.iter().position(|&b| b == b'S').unwrap();
    data[start] = b'.';

    let coord = |o: usize| ((o / stride) as i32, (o % stride) as i32);
    let offset = |(x, y): (i32, i32)| {
        y.rem_euclid(height as i32) as usize * stride + x.rem_euclid(width as i32) as usize
    };

    let start_coord = coord(start);
    assert!(start_coord.0 == width as i32 / 2 && start_coord.1 == height as i32 / 2);

    const STEPS1: usize = 64;
    const STEPS2: usize = 26501365;
    let mut queue = VecDeque::with_capacity(100);
    let mut total1 = 0;

    let mut grid = [[0; 5]; 5];
    let mut visited = HashSet::with_capacity(25 * (width * width));
    let max_steps = start_coord.0 as usize + 2 * width;

    queue.push_back(((max_steps as i32, max_steps as i32), 0));
    while let Some((pos, step)) = queue.pop_front() {
        let off = offset(pos);
        if data[off] != b'.' || !visited.insert(pos) {
            continue;
        }

        if step & 1 == STEPS2 & 1 {
            let grid_pos = (pos.0 as usize / width, pos.1 as usize / height);
            grid[grid_pos.0][grid_pos.1] += 1;
        }
        if step <= STEPS1 && step & 1 == STEPS1 & 1 {
            total1 += 1;
        }

        if step >= max_steps {
            continue;
        }

        queue.push_back(((pos.0 - 1, pos.1), step + 1));
        queue.push_back(((pos.0, pos.1 - 1), step + 1));
        queue.push_back(((pos.0 + 1, pos.1), step + 1));
        queue.push_back(((pos.0, pos.1 + 1), step + 1));
    }

    //println!("{grid:?}");

    fn sq(n: usize) -> usize {
        n * n
    }

    let num_fields = STEPS2 / width;
    let total2 = grid[2][2] * sq(num_fields - 1)
        + grid[2][3] * sq(num_fields)
        + (grid[1][1] + grid[3][1] + grid[1][3] + grid[3][3]) * (num_fields - 1)
        + (grid[1][0] + grid[3][0] + grid[1][4] + grid[3][4]) * num_fields
        + grid[2][0]
        + grid[0][2]
        + grid[4][2]
        + grid[2][4];
    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

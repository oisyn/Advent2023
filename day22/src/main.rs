use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day22")?;

    let mut bricks = Vec::with_capacity(2000);

    for l in input.lines() {
        let mut p = Parser::new(l);
        let x0 = p.parse::<i32>().unwrap();
        let y0 = p.expect(",").parse::<i32>().unwrap();
        let z0 = p.expect(",").parse::<i32>().unwrap();
        let x1 = p.expect("~").parse::<i32>().unwrap();
        let y1 = p.expect(",").parse::<i32>().unwrap();
        let z1 = p.expect(",").parse::<i32>().unwrap();
        bricks.push(((x0, y0, z0), (x1, y1, z1)));
    }

    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));
    let mut safe = vec![true; bricks.len()];
    let mut supporting = vec![Vec::new(); bricks.len()];
    let mut resting = vec![Vec::new(); bricks.len()];

    let mut space = [[(0, 0); 10]; 10];
    let mut set = Vec::with_capacity(100);
    for (idx, b) in bricks.iter().enumerate() {
        let mut max = 0;
        for y in b.0 .1..=b.1 .1 {
            for x in b.0 .0..=b.1 .0 {
                if max < space[y as usize][x as usize].0 {
                    max = space[y as usize][x as usize].0;
                    set.clear();
                    set.push(space[y as usize][x as usize].1)
                } else if max > 0 && max == space[y as usize][x as usize].0 {
                    if !set.contains(&space[y as usize][x as usize].1) {
                        set.push(space[y as usize][x as usize].1);
                    }
                }
            }
        }

        if max > 0 {
            if set.len() == 1 {
                safe[set[0]] = false;
            }
            for &i in &set {
                supporting[i].push(idx);
            }
            resting[idx] = set.clone();
        }

        let h = max + b.1 .2 - b.0 .2 + 1;
        for y in b.0 .1..=b.1 .1 {
            for x in b.0 .0..=b.1 .0 {
                space[y as usize][x as usize] = (h, idx);
            }
        }
    }

    let mut falling = HashSet::with_capacity(2000);
    let mut queue = VecDeque::with_capacity(1000);
    let mut total2 = 0;

    for idx in 0..bricks.len() {
        if safe[idx] {
            continue;
        }

        falling.clear();
        falling.insert(idx);
        queue.clear();
        queue.extend(&supporting[idx]);
        while let Some(idx) = queue.pop_front() {
            if falling.contains(&idx) {
                continue;
            }

            if resting[idx].iter().all(|i| falling.contains(i)) {
                falling.insert(idx);
                total2 += 1;
                queue.extend(&supporting[idx]);
            }
        }
    }

    let total1 = safe.iter().filter(|&&b| b).count();

    drop(input);
    println!("{total1} {total2}");
    Ok(())
}

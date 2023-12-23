use std::collections::{BinaryHeap, HashMap};

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
        bricks.push((
            (x0.min(x1), y0.min(y1), z0.min(z1)),
            (x1.max(x0), y1.max(y0), z1.max(z0)),
        ));
    }

    bricks.sort_unstable_by(|a, b| a.0 .2.cmp(&b.0 .2));

    #[cfg(feature = "validation")]
    {
        for idx in 0..bricks.len() - 1 {
            let b = bricks[idx];
            for b2 in &bricks[idx + 1..] {
                if b2.0 .2 > b.1 .2 {
                    break;
                }
                if !(b.1 .0 < b2.0 .0 || b2.1 .0 < b.0 .0 || b.1 .1 < b2.0 .1 || b2.1 .1 < b.0 .1) {
                    println!("bricks {b:?} and {b2:?} overlap");
                }
            }
        }
        println!("Validation ok!");
    }

    let mut safe = vec![true; bricks.len()];
    let mut supporting = vec![Vec::new(); bricks.len()];
    let mut resting = vec![Vec::new(); bricks.len()];

    let mut space: HashMap<(i32, i32), (i32, usize)> = HashMap::with_capacity(1000);
    let mut set = Vec::with_capacity(100);
    for (idx, b) in bricks.iter().enumerate() {
        let mut max = 0;
        for y in b.0 .1..=b.1 .1 {
            for x in b.0 .0..=b.1 .0 {
                let slot = space.entry((x, y)).or_default();
                if max < slot.0 {
                    max = slot.0;
                    set.clear();
                    set.push(slot.1)
                } else if max > 0 && max == slot.0 {
                    if !set.contains(&slot.1) {
                        set.push(slot.1);
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
            resting[idx].sort_unstable();
        }

        let h = max + b.1 .2 - b.0 .2 + 1;
        for y in b.0 .1..=b.1 .1 {
            for x in b.0 .0..=b.1 .0 {
                space.insert((x, y), (h, idx));
            }
        }
    }

    let mut fall_at = vec![None; bricks.len()];
    let mut num_falls = vec![0; bricks.len()];

    let mut queue: BinaryHeap<usize> = BinaryHeap::with_capacity(100);

    for idx in 0..bricks.len() {
        if resting[idx].is_empty() {
            continue;
        }

        queue.clear();
        queue.extend(&resting[idx]);

        while let Some(i) = queue.pop() {
            match queue.peek() {
                Some(&j) if i == j => continue,
                _ => {}
            }
            if queue.is_empty() {
                fall_at[idx] = Some(i);
                break;
            }

            let Some(j) = fall_at[i] else {
                break;
            };
            queue.push(j);
        }
    }

    for idx in (0..bricks.len()).rev() {
        let Some(i) = fall_at[idx] else {
            continue;
        };
        num_falls[i] += 1 + num_falls[idx];
    }

    let total1 = safe.iter().filter(|&&b| b).count();
    let total2 = num_falls.iter().sum::<usize>();

    drop(input);
    println!("{total1} {total2}");
    Ok(())
}

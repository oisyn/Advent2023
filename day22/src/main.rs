use std::collections::{HashMap, HashSet, VecDeque};

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
        }

        let h = max + b.1 .2 - b.0 .2 + 1;
        for y in b.0 .1..=b.1 .1 {
            for x in b.0 .0..=b.1 .0 {
                space.insert((x, y), (h, idx));
            }
        }
    }

    let mut falling = HashSet::with_capacity(2000);
    let mut staying = HashSet::with_capacity(2000);
    let mut falling_for_brick = vec![Vec::new(); bricks.len()];
    let mut staying_for_brick = vec![Vec::new(); bricks.len()];
    let mut queue = VecDeque::with_capacity(1000);
    let mut total2 = 0;

    for idx in (0..bricks.len()).rev() {
        if safe[idx] {
            continue;
        }

        falling.clear();
        staying.clear();
        falling.insert(idx);
        queue.clear();
        queue.extend(&supporting[idx]);
        while let Some(idx) = queue.pop_front() {
            if falling.contains(&idx) {
                continue;
            }

            if resting[idx].iter().all(|i| falling.contains(i)) {
                falling.insert(idx);

                if !falling_for_brick[idx].is_empty() {
                    falling.extend(&falling_for_brick[idx]);
                    total2 += falling_for_brick[idx].len();
                    queue.extend(&staying_for_brick[idx]);
                } else {
                    total2 += 1;
                    queue.extend(&supporting[idx]);
                }
            } else {
                staying.insert(idx);
            }
        }

        for &i in &supporting[idx] {
            if resting[i][0] == idx {
                falling_for_brick[i] = Vec::new();
                staying_for_brick[i] = Vec::new();
            }
        }

        // let mut v = Vec::from_iter(falling.iter().copied());
        // v.sort();
        // for i in &v {
        //     println!("  {i} falls");
        // }

        #[cfg(not(feature = "test"))]
        {
            falling_for_brick[idx].extend(&falling);
            staying_for_brick[idx].extend(&staying);
        }
    }

    let total1 = safe.iter().filter(|&&b| b).count();

    drop(input);
    println!("{total1} {total2}");
    Ok(())
}

use std::ops::RangeInclusive;

use anyhow::Result;
use util::*;

const fn node_id(s: &str) -> u16 {
    let b = s.as_bytes();
    (b[0] - b'A') as u16 + (b[1] - b'A') as u16 * 26 + (b[2] - b'A') as u16 * 26 * 26
}

#[allow(dead_code)]
fn node_name(id: u16) -> String {
    let b = [
        (id % 26) as u8 + b'A',
        (id / 26 % 26) as u8 + b'A',
        (id / 26_u16.pow(2)) as u8 + b'A',
    ];
    to_str(&b).to_string()
}

fn main() -> Result<()> {
    let input = open_input("day8")?;
    let mut lines = input.lines();

    let path = lines.next().unwrap().as_bytes();
    lines.next();

    const START_RANGE: RangeInclusive<u16> = node_id("AAA")..=node_id("ZZA");
    let mut nodes = vec![[0xffff, 0xffff]; 26_usize.pow(3)];
    let mut start_nodes = Vec::with_capacity(100);

    for l in lines {
        let mut p = Parser::new(l);
        let from = node_id(p.take(3));
        let left = node_id(p.expect(" = (").take(3));
        let right = node_id(p.expect(", ").take(3));
        nodes[from as usize] = [left, right];

        if START_RANGE.contains(&from) {
            start_nodes.push(from);
        }
    }

    const TARGET_NODE: u16 = node_id("ZZZ");
    let mut cur_node = node_id("AAA");
    let mut steps = 0;
    for &p in path.iter().cycle() {
        steps += 1;
        let dir = (p == b'R') as usize;
        cur_node = nodes[cur_node as usize][dir];
        if cur_node == TARGET_NODE {
            break;
        }
    }
    let total1 = steps;

    const TARGET_RANGE: RangeInclusive<u16> = node_id("AAZ")..=node_id("ZZZ");
    let mut total2 = 1;
    for mut n in start_nodes {
        let mut steps = 0_u64;
        for &p in path.iter().cycle() {
            steps += 1;
            let dir = (p == b'R') as usize;
            n = nodes[n as usize][dir];
            if TARGET_RANGE.contains(&n) {
                break;
            }
        }
        total2 = total2 * steps / gcd(total2, steps);
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

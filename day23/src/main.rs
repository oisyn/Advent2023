use std::collections::HashMap;

use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day23")?;
    let data = input.bytes();

    let width = data.iter().position(|&b| is_nl(b)).unwrap();
    let stride = width + 1 + (data[width + 1] == b'\n') as usize;
    let height = (data.len() + 2) / stride;

    let start_tile = 1_i32;
    assert!(data[start_tile as usize] == b'.');
    let end_tile = ((height - 1) * stride + width - 2) as i32;
    assert!(data[end_tile as usize] == b'.');

    let mut node_ids = HashMap::with_capacity(100);

    fn register(
        pos: i32,
        ids: &mut HashMap<i32, i32>,
        nodes1: &mut Vec<Vec<(i32, i32)>>,
        nodes2: &mut Vec<Vec<(i32, i32)>>,
    ) -> i32 {
        let id = *ids.entry(pos).or_insert(nodes1.len() as i32) as usize;
        if nodes1.len() < id + 1 {
            nodes1.resize_with(id + 1, Default::default);
            nodes2.resize_with(id + 1, Default::default);
        }
        id as i32
    }

    let mut nodes1 = Vec::with_capacity(100);
    let mut nodes2 = Vec::with_capacity(100);
    macro_rules! register {
        ($e:expr) => {
            register($e, &mut node_ids, &mut nodes1, &mut nodes2)
        };
    }

    register!(start_tile);
    register!(end_tile);

    let offsets = [1, stride as i32, -1, -(stride as i32)];
    const ONE_WAY_CHARS: [u8; 4] = *b">v<^";
    let valid_range = 0..data.len() as i32;

    let mut queue = Vec::with_capacity(100);
    let mut queued = 0_u64;
    queue.push((start_tile, 1));

    while let Some((start, mut dir)) = queue.pop() {
        let start_id = node_ids[&start];
        let mut pos = start + offsets[dir];
        let mut one_way = pos == start;
        let mut opposite = false;
        let mut len = 1;

        loop {
            let mut num_possible = 0;
            let mut possible = [0; 3];

            for i in 0..4 {
                if i == dir ^ 2 {
                    continue;
                }
                let next = pos + offsets[i];
                if valid_range.contains(&next) && data[next as usize] != b'#' {
                    possible[num_possible] = i;
                    num_possible += 1;
                }
            }

            if num_possible == 1 {
                len += 1;
                dir = possible[0];
                pos = pos + offsets[dir];
                if data[pos as usize] != b'.' {
                    one_way = true;
                    opposite = data[pos as usize] != ONE_WAY_CHARS[dir];
                } else if pos == end_tile {
                    assert!(!one_way || !opposite);
                    nodes1[start_id as usize].push((1, len));
                    nodes2[start_id as usize].push((1, len));
                    break;
                }

                continue;
            }

            // fork
            let id = register!(pos);
            if !one_way || !opposite {
                nodes1[start_id as usize].push((id, len));
            }
            if !one_way || opposite {
                nodes1[id as usize].push((start_id, len));
            }
            nodes2[start_id as usize].push((id, len));
            nodes2[id as usize].push((start_id, len));

            if queued & (1 << id) == 0 {
                queued |= 1 << id;
                for &new_dir in possible.iter().take(num_possible) {
                    queue.push((pos, new_dir));
                }
            }
            break;
        }
    }
    drop(queue);

    fn find_longest(nodes: &Vec<Vec<(i32, i32)>>, start_id: i32, end_id: i32) -> i32 {
        let mut queue = Vec::with_capacity(100);
        let mut total = 0;
        let mut max_len_for = HashMap::with_capacity(10000);
        queue.push((start_id, 0, 1_u64));
        while let Some((start, path_len, done)) = queue.pop() {
            for &(next, len) in &nodes[start as usize] {
                let new_len = path_len + len;
                if done & (1 << next) == 0 {
                    if next == end_id {
                        total = total.max(new_len);
                        continue;
                    }

                    let new_done = done | (1 << next);
                    let e = max_len_for.get_mut(&(next, new_done));
                    if e.map_or(true, |n| *n < new_len) {
                        max_len_for.insert((next, new_done), new_len);
                        queue.push((next, new_len, done | (1 << next)));
                    }
                }
            }
        }
        total
    }

    let total1 = find_longest(&nodes1, 0, 1);
    let total2 = find_longest(&nodes2, 0, 1);

    println!("{total1} {total2}");

    drop(input);
    Ok(())
}

use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use bitvec::prelude::*;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day25")?;

    let mut nodes = Vec::with_capacity(2000);
    let mut node_ids = HashMap::with_capacity(2000);
    let mut node_names = Vec::new();
    let mut connections = Vec::with_capacity(2000);

    fn conn(n0: usize, n1: usize) -> (usize, usize) {
        (n0.min(n1), n0.max(n1))
    }

    for l in input.lines() {
        let mut p = Parser::new(l);

        let n = p.take(3);
        let n_id = *node_ids.entry(n).or_insert(nodes.len());
        if nodes.len() < n_id + 1 {
            nodes.resize_with(n_id + 1, Vec::new);
            node_names.push(n);
        }
        p.expect(": ");
        while !p.at_end() {
            let n2 = p.take(3);
            let n2_id = *node_ids.entry(n2).or_insert(nodes.len());
            if nodes.len() < n2_id + 1 {
                nodes.resize_with(n2_id + 1, Vec::new);
                node_names.push(n2);
            }

            nodes[n_id].push(n2_id);
            nodes[n2_id].push(n_id);
            connections.push(conn(n_id, n2_id));

            p.skip(1);
        }
    }

    let mut queue = VecDeque::with_capacity(nodes.len());
    let mut visited = bitvec![0; nodes.len()];
    let mut conn_weight = Vec::with_capacity(connections.len());

    for &c in &connections {
        queue.clear();
        queue.push_back((0, c.0));
        visited.fill(false);
        visited.set(c.0, true);
        while let Some((step, node)) = queue.pop_front() {
            if step > 0 && node == c.1 {
                conn_weight.push((c, step));
                break;
            }
            for &n in &nodes[node] {
                if (step > 0 || n != c.1) && !visited[n] {
                    visited.set(n, true);
                    queue.push_back((step + 1, n));
                }
            }
        }
    }

    conn_weight.sort_by(|a, b| b.1.cmp(&a.1));
    drop(queue);
    let mut queue = VecDeque::with_capacity(1000);

    'outer: for a in 0..conn_weight.len() - 2 {
        for b in a + 1..conn_weight.len() - 1 {
            for c in b + 1..conn_weight.len() {
                let conns = [conn_weight[a].0, conn_weight[b].0, conn_weight[c].0];

                queue.clear();
                queue.push_back(conns[0].0);
                visited.fill(false);
                visited.set(conns[0].0, true);
                while let Some(node) = queue.pop_front() {
                    for &n in &nodes[node] {
                        let e = conn(node, n);
                        if !conns.contains(&e) && !visited[n] {
                            visited.set(n, true);
                            queue.push_back(n);
                        }
                    }
                }
                if visited.count_ones() != nodes.len() {
                    break 'outer;
                }
            }
        }
    }

    let total1 = visited.count_ones() * (nodes.len() - visited.count_ones());

    drop(input);
    println!("{total1}");

    Ok(())
}

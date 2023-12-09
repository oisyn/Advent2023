use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day9")?;

    let mut total1 = 0;
    let mut total2 = 0;

    for l in input.lines() {
        let mut p = Parser::new(l);
        let mut v = Vec::with_capacity(100);
        v.push(Vec::with_capacity(100));

        while !p.at_end() {
            v[0].push(p.parse::<i64>().unwrap());
            p.skip(1);
        }

        for i in 0_usize.. {
            if i >= v.len() {
                break;
            }

            let d = v[i].windows(2).map(|a| a[1] - a[0]).collect::<Vec<_>>();
            if d.iter().all(|n| *n == 0) {
                break;
            }
            v.push(d);
        }

        total1 += v.iter().map(|d| d.last().unwrap()).sum::<i64>();
        total2 += v.iter().map(|d| d[0]).rev().fold(0, |r, n| n - r);
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

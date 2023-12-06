use anyhow::Result;
use util::*;

fn parse(l: &str) -> Vec<i64> {
    let mut v = 0;
    let mut r = Vec::with_capacity(10);
    for n in to_str(&l.as_bytes()[9..]).split_ascii_whitespace() {
        let num = n.parse::<i64>().unwrap();
        r.push(num);
        v *= 10_i64.pow(n.len() as u32);
        v += num;
    }
    r.push(v);
    r
}

fn main() -> Result<()> {
    let input = open_input("day6")?;
    let mut lines = input.lines();

    let times = parse(lines.next().unwrap());
    let distances = parse(lines.next().unwrap());

    let mut total1 = 1;
    let mut total2 = 0;

    for i in 0..times.len() {
        let t = times[i] as f64;
        let d = distances[i] as f64;

        let det = (t * t - 4.0 * d).sqrt();
        let min = (0.5 * (t - det)).floor() as i64 + 1;
        let max = (0.5 * (t + det)).ceil() as i64 - 1;
        let num = max - min + 1;

        if i < times.len() - 1 {
            total1 *= num;
        } else {
            total2 = num;
        }
    }

    println!("{total1} {total2}");

    Ok(())
}

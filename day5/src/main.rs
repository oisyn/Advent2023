use anyhow::Result;
use util::*;

#[cfg(feature = "u64")]
#[allow(non_camel_case_types)]
type utype = u64;

#[cfg(not(feature = "u64"))]
#[allow(non_camel_case_types)]
type utype = u32;

fn main() -> Result<()> {
    let input = open_input("day5")?;
    let mut lines = input.lines().peekable();

    let mut seeds = Vec::new();
    let mut p = Parser::new(lines.next().unwrap());
    p.expect("seeds: ");
    while p.peek_char().is_some_and(|c| c.is_ascii_digit()) {
        let seed: utype = p.parse().unwrap();
        seeds.push(seed);
        p.skip(1);
    }

    let mut seed_ranges = seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + (c[1] - 1)))
        .collect::<Vec<_>>();

    let mut remapped = vec![0 as utype; seeds.len()];
    let mut remapped_ranges = Vec::new();
    let mut leftover_ranges = Vec::new();

    for _ in 0..7 {
        lines.next();
        lines.next();

        remapped.copy_from_slice(&seeds);

        while lines.peek().is_some_and(|l| !l.is_empty()) {
            let mut p = Parser::new(lines.next().unwrap());
            let dest: utype = p.parse().unwrap();
            p.expect(" ");
            let from: utype = p.parse().unwrap();
            p.expect(" ");
            let count = p.parse::<utype>().unwrap() - 1;
            let to = from + count;
            let dest_to = dest + count;
            let diff = dest.wrapping_sub(from);

            for i in 0..seeds.len() {
                let s = seeds[i];
                if s >= from && s <= to {
                    remapped[i] = s.wrapping_add(diff);
                }
            }

            seed_ranges.retain_mut(|r| {
                if r.0 > to || r.1 < from {
                    // no overlap
                    return true;
                }
                if r.0 >= from {
                    if r.1 <= to {
                        // full range
                        remapped_ranges.push((r.0.wrapping_add(diff), r.1.wrapping_add(diff)));
                        return false;
                    }
                    // first part (r.0 is in, r.1 is out)
                    remapped_ranges.push((r.0.wrapping_add(diff), dest_to));
                    r.0 = to + 1;
                    return true;
                }
                if r.1 > to {
                    // middle part of the range is remapped
                    remapped_ranges.push((dest, dest_to));
                    leftover_ranges.push((to + 1, r.1));
                    r.1 = from - 1;
                }
                // second part (r.0 is out, r.1 is in)
                remapped_ranges.push((dest, r.1.wrapping_add(diff)));
                r.1 = from - 1;
                return true;
            });
            seed_ranges.append(&mut leftover_ranges);
        }

        seed_ranges.append(&mut remapped_ranges);

        #[cfg(feature = "allow_overlaps")]
        {
            (seed_ranges, remapped_ranges) = (remapped_ranges, seed_ranges);
            seed_ranges.clear();
            for &(mut r) in &remapped_ranges {
                seed_ranges.retain(|o| {
                    if o.0 <= r.1 && o.1 >= r.0 {
                        r.0 = r.0.min(o.0);
                        r.1 = r.1.max(o.1);
                        false
                    } else {
                        true
                    }
                });
                seed_ranges.push(r);
            }

            remapped_ranges.clear();
        }

        (seeds, remapped) = (remapped, seeds);
    }

    let part1 = seeds.into_iter().min().unwrap();
    let part2 = seed_ranges.iter().map(|(f, _)| f).min().unwrap();

    println!("{part1} {part2}");

    Ok(())
}

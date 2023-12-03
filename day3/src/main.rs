use std::collections::HashMap;

#[allow(unused_imports)]
use anyhow::{bail, Result};
use util::*;

fn safe_range(d: &[u8], start: usize, end: usize) -> &[u8] {
    let start = (start as isize).clamp(0, d.len() as isize) as usize;
    let end = (end as isize).clamp(0, d.len() as isize) as usize;
    &d[start..end]
}

fn main() -> Result<()> {
    let input = open_input("day3")?;
    let data: &[u8] = input.as_ref();

    let width = data.iter().position(|&c| c == b'\r' || c == b'\n').unwrap();
    let stride = width + 1 + &data[width + 1..].iter().position(|&c| c != b'\n').unwrap();
    let height = (data.len() + 2) / stride;

    #[cfg(feature = "validation")]
    {
        if data.len() != (width - 1) * stride + width {
            bail!("Unexpected input data size");
        }

        if input.lines().any(|l| l.len() != width) {
            bail!("Lines not all of equal size");
        }
    }

    let mut gear_map = HashMap::new();

    let mut total1 = 0;
    let mut total2 = 0;

    let mut y_offset = 0;
    for _ in 0..height {
        let mut x = 0;
        while x < width {
            let c = data[y_offset + x];
            if !c.is_ascii_digit() {
                x += 1;
                continue;
            }

            let start = y_offset + x;
            x += 1;
            while x < width && data[y_offset + x].is_ascii_digit() {
                x += 1;
            }
            let end = y_offset + x;
            x += 1;

            let r = safe_range(
                data,
                start.wrapping_sub(stride + 1),
                end.wrapping_sub(stride - 1),
            )
            .iter()
            .chain(safe_range(data, start.wrapping_sub(1), start))
            .chain(safe_range(data, end, end + 1))
            .chain(safe_range(data, start + stride - 1, end + stride + 1));

            let mut num = 0;
            if r.clone()
                .find(|&&c| c != b'.' && c != b'\n' && c != b'\r' && !c.is_ascii_digit())
                .is_some()
            {
                num = to_str(&data[start..end]).parse::<i32>().unwrap();
                total1 += num;
            }

            for gear in r.filter(|&&c| c == b'*') {
                if let Some(old) = gear_map.insert(gear as *const u8, num) {
                    total2 += old * num;
                }
            }
        }
        y_offset += stride;
    }

    println!("{total1} {total2}");

    Ok(())
}

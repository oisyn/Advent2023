use anyhow::Result;
use itertools::Itertools;
use util::*;

#[cfg(feature = "u128")]
#[allow(non_camel_case_types)]
type utype = u128;

#[cfg(not(feature = "u128"))]
#[allow(non_camel_case_types)]
type utype = u64;

fn main() -> Result<()> {
    let input = open_input("day11")?;
    let data = input.bytes();
    let width = data.iter().position(|&b| b == b'\r' || b == b'\n').unwrap() as utype;
    let stride = width + 1 + (data[width as usize + 1] == b'\n') as utype;
    let height = (data.len() as utype + 2) / stride;

    let mut x_adjust = Vec::with_capacity(100);
    let mut y_adjust = Vec::with_capacity(100);

    for x in 0..width {
        if !data[x as usize..]
            .iter()
            .step_by(stride as usize)
            .contains(&b'#')
        {
            x_adjust.push(x);
        }
    }

    for y in 0..height {
        let o = (y * stride) as usize;
        if !data[o..o + width as usize].contains(&b'#') {
            y_adjust.push(y);
        }
    }

    x_adjust.push(utype::MAX);
    y_adjust.push(utype::MAX);

    let mut xs = Vec::with_capacity(1000);
    let mut ys = Vec::with_capacity(1000);

    for (x, y) in data.iter().filter(|b| **b == b'#').map(|b| {
        let o = unsafe { (b as *const u8).offset_from(data.as_ptr()) } as utype;
        (o % stride, o / stride)
    }) {
        xs.push(x);
        ys.push(y);
    }

    xs.sort();
    // ys is already sorted

    fn calc(coords: &[utype], adjusts: &[utype]) -> (utype, utype) {
        const SIZE: utype = 999_999;

        let mut adjusts = adjusts.iter().copied().skip_while(|&a| a < coords[0]);

        let mut total1 = 0;
        let mut total2 = 0;
        let num_spans = coords.len() - 1;
        for (idx, c) in coords.windows(2).enumerate() {
            let width = c[1] - c[0];
            let adj = adjusts.take_while_ref(|&a| a < c[1]).count() as utype;
            if width == 0 {
                continue;
            }

            let num = ((idx + 1) * (num_spans - idx)) as utype;
            total1 += (width + adj) * num;
            total2 += (width + adj * SIZE) * num;
        }

        (total1, total2)
    }

    let (xtotal1, xtotal2) = calc(&xs, &x_adjust);
    let (ytotal1, ytotal2) = calc(&ys, &y_adjust);

    let (total1, total2) = (xtotal1 + ytotal1, xtotal2 + ytotal2);

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

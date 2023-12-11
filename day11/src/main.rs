use anyhow::Result;
use itertools::Itertools;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day11")?;
    let data = input.bytes();
    let width = data.iter().position(|&b| b == b'\r' || b == b'\n').unwrap() as isize;
    let stride = width + 1 + (data[width as usize + 1] == b'\n') as isize;
    let height = (data.len() as isize + 2) / stride;

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

    struct Galaxy {
        pos: (isize, isize),
        adj: (isize, isize),
    }

    impl Galaxy {
        fn adjust(&self, factor: isize) -> (isize, isize) {
            (
                self.pos.0 + self.adj.0 * factor,
                self.pos.1 + self.adj.1 * factor,
            )
        }

        fn diff(&self, other: &Self, factor: isize) -> usize {
            let a = self.adjust(factor);
            let b = other.adjust(factor);
            a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        }
    }

    let galaxies = data
        .iter()
        .filter(|b| **b == b'#')
        .map(|b| {
            let o = unsafe { (b as *const u8).offset_from(data.as_ptr()) };
            let pos = (o % stride, o / stride);
            let adj = (
                x_adjust.binary_search(&pos.0).unwrap_err() as isize,
                y_adjust.binary_search(&pos.1).unwrap_err() as isize,
            );
            Galaxy { pos, adj }
        })
        .collect_vec();

    let mut total1 = 0;
    let mut total2 = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            total1 += galaxies[i].diff(&galaxies[j], 1);
            total2 += galaxies[i].diff(&galaxies[j], 999999);
        }
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

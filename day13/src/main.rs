use anyhow::Result;
use util::*;

#[derive(Clone, Copy, Default, Debug)]
#[repr(transparent)]
struct Smudged(bool);

impl Smudged {
    pub fn new() -> Self {
        Self(false)
    }

    pub fn eq<'a, I: IntoIterator<Item = &'a u8>>(&mut self, a: I, b: I) -> bool {
        let a = a.into_iter();
        let b = b.into_iter();

        if self.0 {
            return a.eq(b);
        }

        let mut zipped = a.zip(b);
        if zipped.position(|(a, b)| a != b).is_some() {
            self.0 = true;
            zipped.all(|(a, b)| a == b)
        } else {
            true
        }
    }
}

fn main() -> Result<()> {
    let input = open_input("day13")?;
    let mut data = input.bytes();

    let nl_size = {
        let p = data.iter().position(|&b| b == b'\n').unwrap();
        1 + (data[p - 1] == b'\r') as usize
    };
    let nl_char = [b'\n', b'\r'][nl_size - 1];

    let mut total1 = 0;
    let mut total2 = 0;
    while !data.is_empty() {
        let width = data.iter().position(|&b| b == nl_char).unwrap();
        let stride = width + nl_size;
        let new_data;
        let height = if let Some(p) = data.iter().step_by(stride).position(|&b| b == nl_char) {
            new_data = &data[p * stride + nl_size..];
            p
        } else {
            new_data = &[];
            (data.len() + nl_size) / stride
        };
        let field = FieldView::new(data, width, stride, height);
        data = new_data;

        // part 1
        let mut found_y = usize::MAX;
        let mut found_x = usize::MAX;
        'part1: {
            for (y, r) in field.rows().enumerate().skip(1) {
                if r == field.row(y - 1) {
                    if field
                        .rows()
                        .skip(y + 1)
                        .zip(field.rows().take(y - 1).rev())
                        .all(|(a, b)| a == b)
                    {
                        found_y = y;
                        total1 += y * 100;
                        break 'part1;
                    }
                }
            }

            for (x, c) in field.cols().enumerate().skip(1) {
                if c == field.col(x - 1) {
                    if field
                        .cols()
                        .skip(x + 1)
                        .zip(field.cols().take(x - 1).rev())
                        .all(|(a, b)| a == b)
                    {
                        found_x = x;
                        total1 += x;
                        break 'part1;
                    }
                }
            }
        }

        for (y, r) in field.rows().enumerate().skip(1) {
            if y == found_y {
                continue;
            }
            let mut s = Smudged::new();
            if s.eq(r, field.row(y - 1)) {
                if field
                    .rows()
                    .skip(y + 1)
                    .zip(field.rows().take(y - 1).rev())
                    .all(|(a, b)| s.eq(a, b))
                {
                    total2 += y * 100;
                }
            }
        }

        for (x, c) in field.cols().enumerate().skip(1) {
            if x == found_x {
                continue;
            }
            let mut s = Smudged::new();
            if s.eq(c, field.col(x - 1)) {
                if field
                    .cols()
                    .skip(x + 1)
                    .zip(field.cols().take(x - 1).rev())
                    .all(|(a, b)| s.eq(a, b))
                {
                    total2 += x;
                }
            }
        }
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

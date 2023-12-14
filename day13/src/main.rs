use anyhow::Result;
use util::*;

trait SpanEq: Default {
    fn eq<'a, I: IntoIterator<Item = &'a u8>>(&mut self, a: I, b: I) -> bool;

    fn find_mirror<'a, I, Item>(span: I, skip: Option<usize>) -> Option<usize>
    where
        I: Iterator<Item = Item> + ExactSizeIterator + DoubleEndedIterator + Clone,
        Item: IntoIterator<Item = &'a u8> + Clone,
    {
        let skip = skip.unwrap_or(usize::MAX);

        let mut iter = span.clone().enumerate();
        let mut prev = iter.next().unwrap().1;
        iter.find_map(move |(p, cur)| {
            let prev = std::mem::replace(&mut prev, cur.clone());
            let mut eq = Self::default();
            if p != skip && eq.eq(cur, prev) {
                if span
                    .clone()
                    .skip(p + 1)
                    .zip(span.clone().take(p - 1).rev())
                    .all(|(a, b)| eq.eq(a, b))
                {
                    return Some(p);
                }
            }
            None
        })
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct RegularEq;

impl SpanEq for RegularEq {
    fn eq<'a, I: IntoIterator<Item = &'a u8>>(&mut self, a: I, b: I) -> bool {
        a.into_iter().eq(b)
    }
}

#[derive(Clone, Copy, Default, Debug)]
#[repr(transparent)]
struct SmudgedEq(bool);

impl SpanEq for SmudgedEq {
    fn eq<'a, I: IntoIterator<Item = &'a u8>>(&mut self, a: I, b: I) -> bool {
        let a = a.into_iter();
        let b = b.into_iter();

        if self.0 {
            return a.eq(b);
        }

        let mut zipped = a.zip(b);
        if zipped.any(|(a, b)| a != b) {
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
        let mut found_y = None;
        let mut found_x = None;
        if let Some(y) = RegularEq::find_mirror(field.rows(), None) {
            found_y = Some(y);
            total1 += 100 * y;
        } else if let Some(x) = RegularEq::find_mirror(field.cols(), None) {
            found_x = Some(x);
            total1 += x;
        }

        if let Some(y) = SmudgedEq::find_mirror(field.rows(), found_y) {
            total2 += 100 * y;
        } else if let Some(x) = SmudgedEq::find_mirror(field.cols(), found_x) {
            total2 += x;
        }
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

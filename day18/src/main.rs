use anyhow::Result;
use util::*;

#[cfg(feature = "i128")]
#[allow(non_camel_case_types)]
type itype = i128;

#[cfg(not(feature = "i128"))]
#[allow(non_camel_case_types)]
type itype = i64;

fn main() -> Result<()> {
    let input = open_input("day18")?;

    let mut total1 = 0;
    let mut total2 = 0;
    let mut len1 = 0;
    let mut len2 = 0;
    let mut y1 = 0;
    let mut y2 = 0;

    for l in input.lines() {
        let mut p = Parser::new(l);
        let c = p.take_char().unwrap();
        let n = p.expect(" ").parse::<i32>().unwrap();
        len1 += n;

        match c {
            b'R' => total1 += y1 * n,
            b'D' => y1 += n,
            b'L' => total1 -= y1 * n,
            _ => y1 -= n,
        }

        let n = itype::from_str_radix(p.expect(" (#").take(5), 16).unwrap();
        let c = p.take_char().unwrap();
        len2 += n;

        match c {
            b'0' => total2 += y2 * n,
            b'1' => y2 += n,
            b'2' => total2 -= y2 * n,
            _ => y2 -= n,
        }
    }

    total1 = total1.abs() + len1 / 2 + 1;
    total2 = total2.abs() + len2 / 2 + 1;

    println!("{total1} {total2}");

    Ok(())
}

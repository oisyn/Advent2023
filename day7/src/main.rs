use anyhow::Result;
use util::*;

#[cfg(feature = "u64")]
#[allow(non_camel_case_types)]
type utype = u64;

#[cfg(not(feature = "u64"))]
#[allow(non_camel_case_types)]
type utype = u32;

const fn gen_table() -> [u8; 256] {
    let mut t = [0_u8; 256];
    t[b'2' as usize] = 0;
    t[b'3' as usize] = 1;
    t[b'4' as usize] = 2;
    t[b'5' as usize] = 3;
    t[b'6' as usize] = 4;
    t[b'7' as usize] = 5;
    t[b'8' as usize] = 6;
    t[b'9' as usize] = 7;
    t[b'T' as usize] = 8;
    t[b'J' as usize] = 9;
    t[b'Q' as usize] = 10;
    t[b'K' as usize] = 11;
    t[b'A' as usize] = 12;
    t
}

const fn gen_table_j() -> [u8; 256] {
    let mut t = [0_u8; 256];
    t[b'J' as usize] = 0;
    t[b'2' as usize] = 1;
    t[b'3' as usize] = 2;
    t[b'4' as usize] = 3;
    t[b'5' as usize] = 4;
    t[b'6' as usize] = 5;
    t[b'7' as usize] = 6;
    t[b'8' as usize] = 7;
    t[b'9' as usize] = 8;
    t[b'T' as usize] = 9;
    t[b'Q' as usize] = 10;
    t[b'K' as usize] = 11;
    t[b'A' as usize] = 12;
    t
}

const CARD_TABLE: [u8; 256] = gen_table();
const CARD_TABLE_J: [u8; 256] = gen_table_j();

fn parse_ranks(parser: &mut Parser) -> (u32, u32) {
    let b = parser.take(5).as_bytes();
    let mut counts = [0; 13];
    let mut value = 0;
    let mut catvalue = 0;
    let mut countsj = [0; 13];
    let mut maxcountj = 0;
    let mut valuej = 0;
    let mut catvaluej = 0;
    for i in 0..5 {
        let c = CARD_TABLE[b[i] as usize];
        counts[c as usize] += 1;
        catvalue += counts[c as usize];
        value = value * 13 + c as u32;

        let c = CARD_TABLE_J[b[i] as usize];
        countsj[c as usize] += 1;
        catvaluej += countsj[c as usize];
        if c > 0 {
            maxcountj = maxcountj.max(countsj[c as usize]);
        }
        valuej = valuej * 13 + c as u32;
    }

    catvaluej += countsj[0] * maxcountj;

    (
        catvalue as u32 * 371293 + value,
        catvaluej as u32 * 371293 + valuej,
    )
}

fn main() -> Result<()> {
    let input = open_input("day7")?;
    let mut hands = Vec::with_capacity(1000);
    let mut jhands = Vec::with_capacity(1000);

    for l in input.lines() {
        let mut p = Parser::new(l);
        let h = parse_ranks(&mut p);
        p.expect(" ");
        let v: utype = p.parse().unwrap();
        hands.push((h.0, v));
        jhands.push((h.1, v));
    }

    hands.sort_unstable();

    let total1 = hands
        .iter()
        .enumerate()
        .fold(0 as utype, |t, (h, v)| t + (h + 1) as utype * v.1);

    jhands.sort_unstable();
    let total2 = jhands
        .iter()
        .enumerate()
        .fold(0 as utype, |t, (h, v)| t + (h + 1) as utype * v.1);

    println!("{total1} {total2}");

    Ok(())
}

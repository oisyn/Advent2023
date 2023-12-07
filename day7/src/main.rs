use anyhow::Result;
use util::*;

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Category {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Hand(Category, u32);

struct JokeredHands(Hand, Hand);

impl FromParser for JokeredHands {
    fn parse_from<'a>(parser: &mut Parser<'a>) -> Option<Self> {
        let b = parser.take(5).as_bytes();
        let mut counts = [0; 13];
        let mut value = 0;
        let mut countsj = [0; 13];
        let mut valuej = 0;
        for i in 0..5 {
            let c = CARD_TABLE[b[i] as usize];
            counts[c as usize] += 1;
            value = value * 13 + c as u32;

            let c = CARD_TABLE_J[b[i] as usize];
            countsj[c as usize] += 1;
            valuej = valuej * 13 + c as u32;
        }

        counts.sort_unstable_by(|a, b| b.cmp(a));
        countsj[1..].sort_unstable_by(|a, b| b.cmp(a));

        let cat = match (counts[0], counts[1]) {
            (5, _) => Category::FiveOfAKind,
            (4, _) => Category::FourOfAKind,
            (3, 2) => Category::FullHouse,
            (3, _) => Category::ThreeOfAKind,
            (2, 2) => Category::TwoPair,
            (2, _) => Category::Pair,
            _ => Category::HighCard,
        };

        let catj = match (countsj[0] + countsj[1], countsj[2]) {
            (5, _) => Category::FiveOfAKind,
            (4, _) => Category::FourOfAKind,
            (3, 2) => Category::FullHouse,
            (3, _) => Category::ThreeOfAKind,
            (2, 2) => Category::TwoPair,
            (2, _) => Category::Pair,
            _ => Category::HighCard,
        };

        Some(Self(Hand(cat, value), Hand(catj, valuej)))
    }
}

fn main() -> Result<()> {
    let input = open_input("day7")?;
    let mut hands = Vec::with_capacity(1000);
    let mut jhands = Vec::with_capacity(1000);

    for l in input.lines() {
        let mut p = Parser::new(l);
        let h: JokeredHands = p.parse().unwrap();
        p.expect(" ");
        let v: i32 = p.parse().unwrap();
        hands.push((h.0, v));
        jhands.push((h.1, v));
    }

    hands.sort_unstable();

    let total1 = hands
        .iter()
        .enumerate()
        .fold(0_i32, |t, (h, v)| t + (h + 1) as i32 * v.1);

    jhands.sort_unstable();
    let total2 = jhands
        .iter()
        .enumerate()
        .fold(0_i32, |t, (h, v)| t + (h + 1) as i32 * v.1);

    println!("{total1} {total2}");

    Ok(())
}

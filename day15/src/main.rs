use anyhow::Result;
use util::*;

fn main() -> Result<()> {
    let input = open_input("day15")?;
    let data = input.bytes();

    let mut total1 = 0;
    let mut hash = 0_u8;
    let mut label_start = 0;
    const EMPTY: Vec<(&[u8], u8)> = Vec::new();
    let mut boxes = [EMPTY; 256];

    for (p, &b) in data.iter().enumerate() {
        match b {
            b'\r' | b'\n' => {}
            b',' => {
                total1 += hash as u32;
                hash = 0;
                label_start = p + 1;
            }
            _ => {
                if b == b'-' || b == b'=' {
                    let label = &data[label_start..p];
                    let box_vec = &mut boxes[hash as usize];

                    if b == b'-' {
                        if let Some(p) = box_vec.iter().position(|&(s, _)| s == label) {
                            box_vec.remove(p);
                        }
                    } else if let Some(l) = box_vec.iter_mut().find(|&&mut (s, _)| s == label) {
                        l.1 = data[p + 1] - b'0';
                    } else {
                        box_vec.push((label, data[p + 1] - b'0'));
                    }
                }
                hash = hash.wrapping_add(b).wrapping_mul(17);
            }
        }
    }
    total1 += hash as u32;

    let mut total2 = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (p, &(_, f)) in b.iter().enumerate() {
            total2 += (i + 1) * (p + 1) * (f as usize);
        }
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

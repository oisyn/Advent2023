use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use util::*;

#[derive(Default, Debug)]
struct Module<'a> {
    conjunction: bool,
    outputs: Vec<&'a str>,
    fstate: bool,
    cstate: HashMap<&'a str, bool>,
}

fn main() -> Result<()> {
    let input = open_input("day20")?;

    let mut modules: HashMap<&str, Module> = HashMap::new();

    for l in input.lines() {
        let mut p = Parser::new(l);
        let c = p.take_char().unwrap();

        let mut conjunction = false;
        let name;

        if c == b'b' {
            name = "broadcaster";
            p.expect("roadcaster");
        } else {
            conjunction = c == b'&';
            name = p.take_while(|b| b != b' ');
        }
        p.expect(" -> ");

        let mut outputs = Vec::new();

        while !p.at_end() {
            let out = p.take_while(|b| b != b',');
            outputs.push(out);
            modules.entry(out).or_default().cstate.insert(name, false);

            p.skip(2);
        }

        let module = modules.entry(name).or_default();
        module.conjunction = conjunction;
        module.outputs = outputs;
    }
    //dbg!(&modules);

    let mut queue = VecDeque::with_capacity(1000);
    let mut high = 0;
    let mut low = 0;

    let check = {
        let rx = &modules["rx"];
        assert!(rx.cstate.len() == 1);
        *rx.cstate.keys().next().unwrap()
    };
    let mut cycles = vec![(false, 0); modules[check].cstate.len()];
    let mut total1 = 0;
    let mut total2 = 0;
    let mut has_total2 = false;

    'outer: for press in 1.. {
        let module = &modules["broadcaster"];
        low += module.outputs.len() + 1;
        for &o in &module.outputs {
            queue.push_back((o, "broadcaster", false));
        }

        while let Some((m, i, s)) = queue.pop_front() {
            let module = modules.get_mut(m).unwrap();

            if s && m == check {
                let idx = module.cstate.keys().position(|&s| s == i).unwrap();
                if !cycles[idx].0 {
                    cycles[idx] = (true, press);

                    if cycles.iter().all(|c| c.0) {
                        total2 = cycles.iter().fold(1_u64, |t, c| (t / gcd(t, c.1) * c.1));
                        if press > 1000 {
                            break 'outer;
                        }
                        has_total2 = true;
                    }
                }
            }

            let out = if module.conjunction {
                module.cstate.insert(i, s);
                Some(!module.cstate.values().all(|&s| s))
            } else {
                if !s {
                    module.fstate = !module.fstate;
                    Some(module.fstate)
                } else {
                    None
                }
            };

            if let Some(s) = out {
                if s {
                    high += module.outputs.len();
                } else {
                    low += module.outputs.len();
                }
                for &o in &module.outputs {
                    queue.push_back((o, m, s));
                }
            }
        }

        if press == 1000 {
            total1 = high * low;
        }
        if press >= 1000 && has_total2 {
            break;
        }
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

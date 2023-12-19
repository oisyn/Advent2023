use std::collections::HashMap;

use anyhow::Result;
use util::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action<'a> {
    Accept,
    Reject,
    Goto(&'a str),
}

#[derive(Clone, Copy, Debug)]
enum Rule<'a> {
    Direct(Action<'a>),
    Smaller(u8, u32, Action<'a>),
    Larger(u8, u32, Action<'a>),
}

impl<'p> FromParser<'p> for Rule<'p> {
    fn parse_from(p: &mut Parser<'p>) -> Option<Self> {
        let n = p.take_while(|c| !b"<>,}".contains(&c));
        if n.len() > 1 {
            return Some(Rule::Direct(Action::Goto(n)));
        }
        let b = n.as_bytes()[0];
        if b == b'A' {
            return Some(Rule::Direct(Action::Accept));
        } else if b == b'R' {
            return Some(Rule::Direct(Action::Reject));
        }
        let b = b"xmas".iter().position(|&c| b == c)? as u8;
        let c = p.take_char()?;
        let num = p.parse::<u32>()?;
        let action = match p.expect(":").take_while(|c| c != b'}' && c != b',') {
            "A" => Action::Accept,
            "R" => Action::Reject,
            n => Action::Goto(n),
        };

        if c == b'<' {
            Some(Rule::Smaller(b, num, action))
        } else {
            Some(Rule::Larger(b, num, action))
        }
    }
}

fn main() -> Result<()> {
    let input = open_input("day19")?;
    let mut lines = input.lines();

    let mut workflows = HashMap::new();

    for l in lines.by_ref().take_while(|l| !l.is_empty()) {
        let mut p = Parser::new(l);
        let name = p.take_while(|c| c != b'{');
        p.expect("{");

        let mut rules = Vec::new();
        loop {
            rules.push(p.parse::<Rule>().unwrap());
            if p.take_char().unwrap() == b'}' {
                break;
            }
        }

        workflows.insert(name, rules);
    }

    let mut total1 = 0;

    for l in lines {
        let mut p = Parser::new(l);
        let part = [
            p.expect("{x=").parse::<u32>().unwrap(),
            p.expect(",m=").parse::<u32>().unwrap(),
            p.expect(",a=").parse::<u32>().unwrap(),
            p.expect(",s=").parse::<u32>().unwrap(),
        ];

        let mut workflow = &workflows["in"];
        loop {
            let action = 'action: {
                for &r in workflow {
                    match r {
                        Rule::Direct(a) => break 'action a,
                        Rule::Smaller(c, v, a) => {
                            if part[c as usize] < v {
                                break 'action a;
                            }
                        }
                        Rule::Larger(c, v, a) => {
                            if part[c as usize] > v {
                                break 'action a;
                            }
                        }
                    }
                }
                unreachable!();
            };

            workflow = match action {
                Action::Accept => {
                    total1 += part.iter().sum::<u32>();
                    break;
                }
                Action::Reject => break,
                Action::Goto(wf) => &workflows[wf],
            }
        }
    }

    let mut total2 = 0;

    let mut queue = Vec::with_capacity(10000);
    queue.push(([1..=4000, 1..=4000, 1..=4000, 1..=4000], Action::Goto("in")));
    while !queue.is_empty() {
        let (mut part, mut action) = queue.pop().unwrap();

        loop {
            let workflow = match action {
                Action::Accept => {
                    total2 += part
                        .iter()
                        .fold(1_u64, |s, r| s * (r.end() - r.start() + 1) as u64);
                    break;
                }
                Action::Reject => break,
                Action::Goto(wf) => &workflows[wf],
            };

            action = 'action: {
                for &r in workflow {
                    match r {
                        Rule::Direct(a) => break 'action a,
                        Rule::Smaller(c, v, a) => {
                            let r = part[c as usize].clone();
                            if *r.end() < v {
                                break 'action a;
                            } else if *r.start() < v {
                                let mut new_part = part.clone();
                                new_part[c as usize] = *r.start()..=v - 1;
                                queue.push((new_part, a));
                                part[c as usize] = v..=*r.end();
                            }
                        }
                        Rule::Larger(c, v, a) => {
                            let r = part[c as usize].clone();
                            if *r.start() > v {
                                break 'action a;
                            } else if *r.end() > v {
                                let mut new_part = part.clone();
                                new_part[c as usize] = v + 1..=*r.end();
                                queue.push((new_part, a));
                                part[c as usize] = *r.start()..=v;
                            }
                        }
                    }
                }
                unreachable!();
            };
        }
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

use std::collections::HashMap;

use anyhow::Result;
use util::*;

#[derive(Clone, Copy)]
struct Rule<'a> {
    action: &'a str,
    t: u8,
    c: u8,
    v: u32,
}

impl<'a> std::fmt::Display for Rule<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.t == 0 {
            write!(f, "{}", self.action)
        } else {
            write!(
                f,
                "{}{}{}:{}",
                b"xmas"[self.c as usize] as char, self.t as char, self.v, self.action
            )
        }
    }
}

impl<'a> std::fmt::Debug for Rule<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl<'a> Rule<'a> {
    fn direct(action: &'a str) -> Self {
        Self {
            action,
            t: 0,
            c: 0,
            v: 0,
        }
    }

    fn compare(action: &'a str, t: u8, c: u8, v: u32) -> Self {
        Self { action, t, c, v }
    }
}

impl<'p> FromParser<'p> for Rule<'p> {
    fn parse_from(p: &mut Parser<'p>) -> Option<Self> {
        let n = p.take_while(|c| !b"<>,}".contains(&c));
        if n.len() > 1 {
            return Some(Rule::direct(n));
        }
        let b = n.as_bytes()[0];
        if {
            let pc = p.peek_char().unwrap();
            pc == b',' || pc == b'}'
        } {
            return Some(Rule::direct(n));
        }

        let b = b"xmas".iter().position(|&c| b == c)? as u8;
        let c = p.take_char()?;
        let num = p.parse::<u32>()?;
        let action = p.expect(":").take_while(|c| c != b'}' && c != b',');

        Some(Rule::compare(action, c, b, num))
    }
}

fn optimize(workflows: &mut HashMap<&str, Vec<Rule>>) {
    let mut rename = HashMap::new();

    loop {
        let mut changed = false;

        for &name in rename.keys() {
            if name != "in" {
                workflows.remove(name);
            }
        }

        for (&name, rules) in workflows.iter_mut() {
            for r in rules.iter_mut() {
                while let Some(&new) = rename.get(r.action) {
                    r.action = new;
                }
            }

            let last = rules[rules.len() - 1].action;
            while rules.len() > 1 {
                if rules[rules.len() - 2].action == last {
                    rules.remove(rules.len() - 2);
                } else {
                    break;
                }
            }

            if rules.len() == 1 {
                rename.insert(name, last);
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    // for (&name, rules) in workflows.iter_mut() {
    //     println!("{name}{rules:?}");
    // }
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
            rules.push(p.parse::<Rule>().expect(l));
            if p.take_char().unwrap() == b'}' {
                break;
            }
        }

        workflows.insert(name, rules);
    }

    optimize(&mut workflows);

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
                        Rule {
                            action: a, t: 0, ..
                        } => break 'action a,
                        Rule {
                            action: a,
                            t: b'<',
                            c,
                            v,
                        } => {
                            if part[c as usize] < v {
                                break 'action a;
                            }
                        }
                        Rule {
                            action: a,
                            t: _,
                            c,
                            v,
                        } => {
                            if part[c as usize] > v {
                                break 'action a;
                            }
                        }
                    }
                }
                unreachable!();
            };

            if action == "A" {
                total1 += part.iter().sum::<u32>();
                break;
            }
            if action == "R" {
                break;
            }
            workflow = &workflows.get(action).expect(action);
        }
    }

    let mut total2 = 0;

    let mut queue = Vec::with_capacity(10000);
    queue.push(([(1, 4000); 4], "in"));
    while !queue.is_empty() {
        let (mut part, mut action) = queue.pop().unwrap();

        loop {
            if action == "A" {
                total2 += part.iter().fold(1_u64, |s, r| s * (r.1 - r.0 + 1) as u64);
                break;
            }
            if action == "R" {
                break;
            }
            let workflow = &workflows[action];

            action = 'action: {
                for &r in workflow {
                    match r {
                        Rule {
                            action: a, t: 0, ..
                        } => break 'action a,
                        Rule {
                            action: a,
                            t: b'<',
                            c,
                            v,
                        } => {
                            let r = part[c as usize];
                            if r.1 < v {
                                break 'action a;
                            } else if r.0 < v {
                                let mut new_part = part;
                                new_part[c as usize] = (r.0, v - 1);
                                queue.push((new_part, a));
                                part[c as usize] = (v, r.1);
                            }
                        }
                        Rule {
                            action: a,
                            t: _,
                            c,
                            v,
                        } => {
                            let r = part[c as usize].clone();
                            if r.0 > v {
                                break 'action a;
                            } else if r.1 > v {
                                let mut new_part = part;
                                new_part[c as usize] = (v + 1, r.1);
                                queue.push((new_part, a));
                                part[c as usize] = (r.0, v);
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

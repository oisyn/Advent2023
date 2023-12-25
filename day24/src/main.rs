use std::collections::HashSet;

use anyhow::Result;
use util::*;

trait DotProduct {
    fn dot(self, other: Self) -> i64;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl DotProduct for Vec2 {
    fn dot(self, other: Self) -> i64 {
        self.x * other.x + self.y * other.y
    }
}

impl std::ops::Index<usize> for Vec2 {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        [&self.x, &self.y][index]
    }
}

const fn vec2(x: i64, y: i64) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    pub fn to_vec2(self) -> Vec2 {
        vec2(self.x, self.y)
    }
}

impl DotProduct for Vec3 {
    fn dot(self, other: Self) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        [&self.x, &self.y, &self.z][index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        [&mut self.x, &mut self.y, &mut self.z][index]
    }
}

const fn vec3(x: i64, y: i64, z: i64) -> Vec3 {
    Vec3 { x, y, z }
}

#[derive(Copy, Clone, Debug)]
struct Ray {
    pos: Vec3,
    dir: Vec3,
}

fn main() -> Result<()> {
    let input = open_input("day24")?;

    let mut rays = Vec::with_capacity(1000);

    for l in input.lines() {
        let mut p = Parser::new(l);
        let x = p.parse::<i64>().unwrap();
        let y = p.expect(", ").parse::<i64>().unwrap();
        let z = p.expect(", ").parse::<i64>().unwrap();
        let vx = p.expect(" @ ").parse::<i64>().unwrap();
        let vy = p.expect(", ").parse::<i64>().unwrap();
        let vz = p.expect(", ").parse::<i64>().unwrap();

        rays.push(Ray {
            pos: vec3(x, y, z),
            dir: vec3(vx, vy, vz),
        });
    }

    let mut total1 = 0;

    let mut same_speeds = [Vec::new(), Vec::new(), Vec::new()];

    for i in 0..rays.len() - 1 {
        let r0 = rays[i];
        let n0 = vec2(r0.dir.y, -r0.dir.x);
        let d0 = r0.pos.to_vec2().dot(n0);
        for j in i + 1..rays.len() {
            let r1 = rays[j];

            for a in 0..3 {
                if r0.dir[a] == r1.dir[a] {
                    same_speeds[a].push((i, j));
                }
            }

            let n1 = vec2(r1.dir.y, -r1.dir.x);
            let d1 = r1.pos.to_vec2().dot(n1);

            let d = n0.dot(r1.dir.to_vec2());
            if d == 0 {
                //println!("{i} and {j} don't intersect (parallel) ({r0:?}, {r1:?})");
                continue;
            }
            let s = n0.dot(r1.pos.to_vec2());
            let dir = d > 0;
            let side = s < d0;
            if dir != side {
                //println!("{i} and {j} don't intersect");
                continue;
            }

            let dir = n1.dot(r0.dir.to_vec2()) > 0;
            let side = n1.dot(r0.pos.to_vec2()) < d1;
            if dir != side {
                //println!("{i} and {j} don't intersect");
                continue;
            }

            let t = (d0 - s) as i128 / d as i128;
            let p = (
                r1.pos.x as i128 + t * r1.dir.x as i128,
                r1.pos.y as i128 + t * r1.dir.y as i128,
            );
            //println!("intersection at {p:?}");

            let range = 200000000000000..=400000000000000;
            if range.contains(&p.0) && range.contains(&p.1) {
                total1 += 1;
            }
        }
    }

    // determine possible speeds
    let mut speed_sets = [HashSet::new(), HashSet::new(), HashSet::new()];
    for axis in 0..3 {
        let mut first = true;
        assert!(!same_speeds[axis].is_empty());
        for &(i, j) in &same_speeds[axis] {
            let r0 = rays[i];
            let r1 = rays[j];
            let dist = (r0.pos[axis] - r1.pos[axis]).abs();
            let v = r0.dir[axis];

            if first {
                // as r0 and r1 move at the same speed, any possible speed (relative to r0's speed)
                // is a divisor of their difference in position (positive and negative).
                let divisors = (1..)
                    .take_while(move |i| i * i <= dist)
                    .filter(move |i| dist % i == 0)
                    .flat_map(move |i| [i, dist / i]);

                speed_sets[axis] = HashSet::from_iter(divisors.flat_map(|d| [v + d, v - d]));
            } else {
                // for every subsequent pair, test if the speed relative to these rays's speed is
                // a divisor for the difference in position
                speed_sets[axis].retain(|&s| (s - v) != 0 && dist % (s - v) == 0);
            }
            assert!(!speed_sets.is_empty());
            first = false;
            if speed_sets.len() == 1 {
                break;
            }
        }
    }

    // test with the smallest set
    let min_axis = (0..3)
        .min_by(|&a, &b| speed_sets[a].len().cmp(&speed_sets[b].len()))
        .unwrap();

    let mut total2 = 0;
    'outer: for &s in &speed_sets[min_axis] {
        let (i, j) = same_speeds[min_axis][0];
        let r0 = rays[i];
        let r1 = rays[j];
        // println!(
        //     "checking r0={r0:?} and r1={r1:?} at axis = {}",
        //     b"xyz"[min_axis] as char
        // );
        let v = r0.dir[min_axis];
        let d = r1.pos[min_axis] - r0.pos[min_axis];
        let dt = d / (s - v); // delta time between the two hits
                              // println!("s = {s}, dt = {dt}");

        let mut other_axis = (min_axis + 1) % 3;
        if r0.dir[other_axis] == r1.dir[other_axis] {
            other_axis = (min_axis + 2) % 3;
            assert!(r0.dir[other_axis] != r1.dir[other_axis]);
        }

        // println!("other_axis = {}", b"xyz"[other_axis] as char);

        let x0 = r0.pos[other_axis];
        let v0 = r0.dir[other_axis];
        let x1 = r1.pos[other_axis];
        let v1 = r1.dir[other_axis];

        'inner: for &s in &speed_sets[other_axis] {
            // solve for t0 using delta time on another axis
            // x + s*t0 = x0 + v0*t0
            // x + s*t1 = x1 + v1*t1
            // t1 - t0 = dt  ==>
            //
            // x + s*t1 - x - s*t0 = x1 + v1*t1 - x0 - v0*t0
            // s*(t1-t0) = x1 - x0 + v1*t1 - v0*t0
            // s*dt = x1 - x0 + v1*(t0+dt) - v0*t0
            // s*dt = x1 - x0 + v1*t0 + v1*dt - v0*t0
            // s*dt = (v1-v0)*t0 + x1 - x0 + v1*dt
            // s*dt - x1 + x0 - v1*dt = (v1-v0)*t0
            // ((s-v1)*dt - x1 + x0)/(v1-v0) = t0

            // println!("s = {s}");

            let t0_n = (s - v1) * dt - x1 + x0;
            // println!("t0_n = {t0_n}, dv = {}", v1 - v0);
            // t0_n should be divisible by (v1-v0)
            if t0_n % (v1 - v0) != 0 {
                // println!("not divisible");
                continue;
            }
            let t0 = t0_n / (v1 - v0);
            let t1 = t0 + dt;
            // println!("found t0={t0}, t1={t1}");

            let mut pos = [0; 3];
            let mut speed = [s; 3];
            for i in 0..3 {
                if i != other_axis {
                    speed[i] = ((r1.pos[i] + r1.dir[i] * t1) - (r0.pos[i] + r0.dir[i] * t0)) / dt;
                }
                pos[i] = r0.pos[i] + (r0.dir[i] - speed[i]) * t0;
            }
            // println!("potential throw at {pos:?} with speed {speed:?}");

            // println!("checking other rays");
            for &r in &rays {
                for i in 0..3 {
                    if speed[i] == r.dir[i] {
                        if pos[i] != r.pos[i] {
                            // println!("bail because of parallel");
                            continue 'inner;
                        }
                    } else {
                        let t = (r.pos[i] - pos[i]) / (speed[i] - r.dir[i]);
                        if pos[i] + t * speed[i] != r.pos[i] + t * r.dir[i] {
                            // println!("bail because no intersection");
                            continue 'inner;
                        }
                    }
                }
            }

            // println!("throw at {pos:?}");

            total2 = pos.iter().sum();
            break 'outer;
        }
    }

    drop(input);
    println!("{total1} {total2}");

    Ok(())
}

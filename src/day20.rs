use num::Zero;
use std::{cmp::Ordering, num::NonZeroI32};

framework::day!(20, parse => pt1, pt2);

fn pt1(particles: &[Particle]) -> Result<usize> {
    particles
        .iter()
        .map(|p| p.acc.x.abs() + p.acc.y.abs() + p.acc.z.abs())
        .enumerate()
        .fold(Err(i32::MAX), |state, (index, abs_acc)| match state {
            Ok((prev_abs_acc, prev_index)) => match abs_acc.cmp(&prev_abs_acc) {
                Ordering::Less => Ok((abs_acc, index)),
                Ordering::Equal => Err(abs_acc),
                Ordering::Greater => Ok((prev_abs_acc, prev_index)),
            },
            Err(prev_abs_acc) => match abs_acc.cmp(&prev_abs_acc) {
                Ordering::Less => Ok((abs_acc, index)),
                _ => Err(prev_abs_acc),
            },
        })
        .map(|(_, index)| index)
        .ok()
        .ok_or(Error::InvalidInput(
            "multiple particles have the same absolute acceleration",
        ))
}

// On my input, 39 iterations of brute-force is all it takes to get rid of all
// collisions, which means there's very little point in doing an exact solution.
//
// The brute-force up to 50 cycles, as shown here, takes 0.6ms.
// The exact solution, without doing the brute-force first, takes 10.8ms.
// The exact solution after doing the brute force takes 4.9ms.
//
// That is because the exact solution is O(n^2) over the particle count, whereas
// in practice, because the times at which collisions are found are so early-on,
// that brute force is simply a lot faster.
fn pt2(particles: &[Particle]) -> usize {
    let mut particles = particles.to_vec();
    let mut collision_map = HashMap::<Vec3, usize>::new();
    for _ in 0..50 {
        for particle in &mut particles {
            particle.vel += particle.acc;
            particle.pos += particle.vel;
        }

        particles
            .iter()
            .for_each(|p| *collision_map.entry(p.pos).or_default() += 1);
        particles.retain(|p| *collision_map.get(&p.pos).unwrap() == 1);
        collision_map.clear();
    }

    pt2_exact(&particles)
}

fn pt2_exact(particles: &[Particle]) -> usize {
    // Each component of the position of a particle can be represented by a
    // parabola:
    // f(x) = (a/2)x^2 + (v+a/2)x + p
    //
    // This ballistic motion has its initial velocity slightly skewed due to the
    // numerical integration used, where vel += acc, and then pos += vel.
    //
    // We can use this to find the intersection between two particles, which is
    // simply the root of the difference. Each pair of particles only has up to
    // two chances to collide, once for each real root, but only ends up
    // colliding under two circumstances:
    // - It has not been destroyed previously
    // - It collides at an integer value of x > 0.
    // - It collides at the same integer value for all components.
    //
    // To determine which collisions actually happen, we start by collecting all
    // the potential collisions, order them by timestamp X, and remove particles
    // only when both particles involved in the collision are still alive.

    #[derive(Debug)]
    struct CollisionInfo {
        at: NonZeroI32,
        p1: usize,
        p2: usize,
    }

    let mut potential_collisions = Vec::new();
    for ((p1_idx, p1), (p2_idx, p2)) in particles.iter().enumerate().tuple_combinations() {
        let roots = get_valid_roots(p1, p2, |v| v.x)
            .combine_with(|| get_valid_roots(p1, p2, |v| v.y))
            .combine_with(|| get_valid_roots(p1, p2, |v| v.z));
        match roots {
            Roots::None => continue,
            Roots::One(x) | Roots::Two(x, _) => potential_collisions.push(CollisionInfo {
                at: x,
                p1: p1_idx,
                p2: p2_idx,
            }),
            Roots::Infinite => potential_collisions.push(CollisionInfo {
                at: NonZeroI32::new(1).unwrap(),
                p1: p1_idx,
                p2: p2_idx,
            }),
        }
    }

    if potential_collisions.is_empty() {
        return particles.len();
    }

    potential_collisions.sort_unstable_by_key(|c| c.at);

    let mut collided_at = vec![None; particles.len()];
    for collision in &potential_collisions {
        #[rustfmt::skip]
        if     !matches!(collided_at[collision.p1], Some(x) if x < collision.at)
            && !matches!(collided_at[collision.p2], Some(x) if x < collision.at)
        {
            collided_at[collision.p1] = Some(collision.at);
            collided_at[collision.p2] = Some(collision.at);
        }
    }

    collided_at.iter().filter(|c| c.is_none()).count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Roots {
    None,
    One(NonZeroI32),
    Two(NonZeroI32, NonZeroI32),
    Infinite,
}

const EPSILON: f32 = f32::EPSILON * 8.0;

fn get_valid_time(r: f32) -> Option<NonZeroI32> {
    if r < 1.0 - EPSILON {
        return None;
    }
    let rounded = r.round();
    if (r - rounded).abs() > EPSILON {
        return None;
    }
    NonZeroI32::new(rounded as i32)
}

fn get_valid_roots<F: Fn(&Vec3) -> i32>(p1: &Particle, p2: &Particle, component: F) -> Roots {
    let a = (component(&p1.acc) as f32 - component(&p2.acc) as f32) / 2.0;
    let b = (component(&p1.vel) as f32 - component(&p2.vel) as f32) + a;
    let c = component(&p1.pos) as f32 - component(&p2.pos) as f32;
    let mut res = [None, None];
    if a.is_zero() {
        if b.is_zero() {
            return if c.is_zero() {
                Roots::Infinite
            } else {
                Roots::None
            };
        }

        res[0] = get_valid_time(-c / b);
    } else {
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            return Roots::None;
        }
        let sqrt_d = d.sqrt();
        let sqrt_d2 = sqrt_d * 2.0;
        if (sqrt_d2 - sqrt_d2.round()).abs() > EPSILON {
            return Roots::None;
        }

        if sqrt_d == 0.0 {
            res[0] = get_valid_time(-b / (2.0 * a));
        } else {
            for (i, r) in [(-b - sqrt_d) / (2.0 * a), (-b + sqrt_d) / (2.0 * a)]
                .into_iter()
                .enumerate()
            {
                res[i] = get_valid_time(r);
            }
        }
    }
    match res {
        [None, None] => Roots::None,
        [Some(x), None] => Roots::One(x),
        [None, Some(x)] => Roots::One(x),
        [Some(a), Some(b)] => match a.cmp(&b) {
            Ordering::Equal => Roots::One(a),
            Ordering::Less => Roots::Two(a, b),
            Ordering::Greater => Roots::Two(b, a),
        },
    }
}

impl Roots {
    fn combine_with<F: FnOnce() -> Roots>(self, next: F) -> Roots {
        match self {
            Roots::None => Roots::None,
            Roots::One(s1) => {
                let next = next();
                match next {
                    Roots::Infinite => self,
                    Roots::One(n1) if n1 == s1 => self,
                    Roots::Two(n1, n2) if n1 == s1 || n2 == s1 => self,
                    _ => Roots::None,
                }
            }
            Roots::Two(s1, s2) => {
                let next = next();
                match next {
                    Roots::One(n1) if s1 == n1 || s2 == n1 => next,
                    Roots::Two(n1, n2) if s1 == n1 && s2 == n2 => self,
                    Roots::Two(n1, n2) if s1 == n1 || s1 == n2 => Roots::One(s1),
                    Roots::Two(n1, n2) if s2 == n1 || s2 == n2 => Roots::One(s2),
                    Roots::Infinite => self,
                    _ => Roots::None,
                }
            }
            Roots::Infinite => next(),
        }
    }
}

type Vec3 = framework::vecs::Vec3<i32>;

#[derive(Debug, Clone)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

fn parse(input: &[u8]) -> Result<Vec<Particle>> {
    use parsers::*;
    let nr = number::<i32>();
    let cnr = token(b',').then(nr);
    let vec3 = token(b'<').then(nr).and(cnr).and(cnr).trailed(token(b'>'));
    let vec3 = vec3.map(|((x, y), z)| Vec3::new(x, y, z));
    let pos = token(b"p=").then(vec3);
    let vel = token(b"v=").then(vec3);
    let acc = token(b"a=").then(vec3);
    let particle = pos
        .and(token(b", ").then(vel))
        .and(token(b", ").then(acc))
        .map(|((pos, vel), acc)| Particle { pos, vel, acc });
    particle.sep_by(token(b'\n')).execute(input)
}

tests! {
    test_pt!(parse, pt1, b"\
p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>
" => 0);
    test_pt!(parse, pt2, b"\
p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>
" => 1);
}

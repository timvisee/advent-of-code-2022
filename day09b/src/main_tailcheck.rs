// An implementation that attempts to process a full move in a single iteration, switching to
// step-by-step when the tail moves. Sadly, this doesn't seem to be any faster.

use std::collections::HashSet;

pub fn main() {
    let cmds = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| (l[0], atoi::atoi(&l[2..]).unwrap()));

    let mut knots: [(isize, isize); 10] = Default::default();
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    for (d, l) in cmds {
        let d = match d {
            b'U' => (0, -1),
            b'D' => (0, 1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => unreachable!(),
        };

        let mut tail_moves = false;
        let mut knots_check = knots;
        let h = &mut knots_check[0];
        *h = (h.0 + d.0 * l, h.1 + d.1 * l);
        for i in 1..10 {
            let (h, t) = knots_check.split_at_mut(i);
            let h = &mut h[i - 1];
            let t = &mut t[0];

            if !adjacent(*h, *t) {
                if i == 5 {
                    tail_moves = true;
                    break;
                }

                let d = (t.0 - h.0, t.1 - h.1);
                let l = d.0.abs().max(d.1.abs());
                let m = (d.0 / l, d.1 / l);
                *t = (h.0 + m.0, h.1 + m.1);
            } else {
                break;
            }
        }

        if tail_moves {
            for _ in 0..l {
                let h = &mut knots[0];
                *h = (h.0 + d.0, h.1 + d.1);

                for i in 1..10 {
                    let (h, t) = knots.split_at_mut(i);
                    let h = &mut h[i - 1];
                    let t = &mut t[0];

                    if !adjacent(*h, *t) {
                        let d = (t.0 - h.0, t.1 - h.1);
                        let l = d.0.abs().max(d.1.abs());
                        let m = (d.0 / l, d.1 / l);
                        *t = (h.0 + m.0, h.1 + m.1);
                        if i == 9 {
                            seen.insert(*t);
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            knots = knots_check;
        }
    }

    println!("{}", seen.len());
}

#[inline(always)]
fn adjacent(h: (isize, isize), t: (isize, isize)) -> bool {
    h.0.abs_diff(t.0) <= 1 && h.1.abs_diff(t.1) <= 1
}

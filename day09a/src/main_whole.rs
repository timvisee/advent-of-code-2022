// An implementation that attempts to process a full move in a single iteration.
// Sadly, this doesn't seem to be any faster.

use std::collections::HashSet;

pub fn main() {
    let cmds = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| match (l[0], atoi::atoi::<i32>(&l[2..]).unwrap()) {
            (b'U', l) => ((0, -1), l),
            (b'D', l) => ((0, 1), l),
            (b'L', l) => ((-1, 0), l),
            (_, l) => ((1, 0), l),
        });
    let (mut h, mut t, mut seen): ((i32, i32), (i32, i32), HashSet<_>) = Default::default();
    seen.insert((0, 0));

    for (d, l) in cmds {
        h = (h.0 + d.0 * l, h.1 + d.1 * l);
        if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
            let to = (h.0 - d.0, h.1 - d.1);
            while t != to {
                t.0 += (to.0 - t.0).signum();
                t.1 += (to.1 - t.1).signum();
                seen.insert(t);
            }
        }
    }

    println!("{}", seen.len());
}

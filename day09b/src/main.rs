pub fn main() {
    let cmds = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| match (l[0], atoi::atoi(&l[2..]).unwrap()) {
            (b'U', l) => ((0, -1), l),
            (b'D', l) => ((0, 1), l),
            (b'L', l) => ((-1, 0), l),
            (_, l) => ((1, 0), l),
        });
    let (mut knots, mut s): ([(i32, i32); 10], rustc_hash::FxHashSet<_>) = Default::default();
    s.insert((0, 0));

    for (d, l) in cmds {
        for _ in 0..l {
            knots[0].0 += d.0;
            knots[0].1 += d.1;

            for i in 1..10 {
                let (h, t) = knots.split_at_mut(i);
                let (h, t) = (h[i - 1], &mut t[0]);
                if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                    let d = (t.0 - h.0, t.1 - h.1);
                    let l = d.0.abs().max(d.1.abs());
                    let m = (d.0 / l, d.1 / l);
                    *t = (h.0 + m.0, h.1 + m.1);
                    (i == 9).then(|| s.insert(*t));
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", s.len());
}

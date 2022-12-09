pub fn main() {
    let cmds = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| match (l[0], atoi::atoi(&l[2..]).unwrap()) {
            (b'U', l) => ((0, -1), l),
            (b'D', l) => ((0, 1), l),
            (b'L', l) => ((-1, 0), l),
            (_, l) => ((1, 0), l),
        });
    let (mut h, mut t, mut s): ((i32, i32), (_, _), rustc_hash::FxHashSet<_>) = Default::default();
    s.insert((0, 0));

    for (d, l) in cmds {
        for _ in 0..l {
            h = (h.0 + d.0, h.1 + d.1);
            if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                t = (h.0 - d.0, h.1 - d.1);
                s.insert(t);
            }
        }
    }

    println!("{}", s.len());
}

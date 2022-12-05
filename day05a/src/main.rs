use itertools::Itertools;

const STACKS: usize = 9;

pub fn main() {
    let d = include_bytes!("../input.txt");
    let (b, m) = d.split_at(d.windows(2).position(|b| b == b"\n\n").unwrap() + 2);
    let mut s: [Vec<u8>; STACKS] = Default::default();

    b.split(|b| b == &b'\n').rev().skip(1).for_each(|l| {
        l.iter()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| c != &&b' ')
            .for_each(|(i, c)| s[i].push(*c))
    });

    m.split(|b| b == &b'\n').for_each(|m| {
        let (n, a, b): (usize, _, _) = m
            .split(|b| b == &b' ')
            .skip(1)
            .step_by(2)
            .map(|n| atoi::atoi(n).unwrap())
            .collect_tuple()
            .unwrap();
        for _ in 0..n {
            let tmp = s[a - 1].pop().unwrap();
            s[b - 1].push(tmp);
        }
    });

    s.iter()
        .for_each(|s| print!("{}", *s.last().unwrap() as char));
    println!();
}

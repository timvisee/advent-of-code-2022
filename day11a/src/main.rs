#![feature(box_syntax)]

use itertools::Itertools;

struct Monkey {
    bag: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    div: usize,
    yay: usize,
    nay: usize,
    ins: usize,
}

pub fn main() {
    let mut m: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|m| {
            let l: Vec<_> = m.lines().map(|l| l.split(": ").last().unwrap()).collect();
            Monkey {
                bag: l[1].split(", ").map(|n| n.parse().unwrap()).collect(),
                op: {
                    let op: Vec<_> = l[2].rsplit_once("= ").unwrap().1.split(' ').collect();
                    match op[2] {
                        "old" => box |old| old * old,
                        b => match (op[1], b.parse::<usize>().unwrap()) {
                            ("+", n) => box move |old| old + n,
                            ("*", n) => box move |old| old * n,
                            _ => unreachable!(),
                        },
                    }
                },
                div: l[3].rsplit_once(' ').unwrap().1.parse().unwrap(),
                yay: l[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
                nay: l[5].rsplit_once(' ').unwrap().1.parse().unwrap(),
                ins: 0,
            }
        })
        .collect();
    let (mo, mut bags): (usize, _) = (m.iter().map(|m| m.div).product(), vec![vec![]; m.len()]);

    (0..20).for_each(|_| {
        m.iter_mut().enumerate().for_each(|(i, m)| {
            m.bag.append(&mut bags[i]);
            m.bag.drain(0..).for_each(|mut n| {
                n = (m.op)(n) / 3 % mo;
                bags[if n % m.div == 0 { m.yay } else { m.nay }].push(n);
                m.ins += 1;
            });
        });
    });

    println!(
        "{}",
        m.iter()
            .map(|m| m.ins)
            .sorted_unstable_by(|a, b| b.cmp(a))
            .take(2)
            .product::<usize>()
    );
}

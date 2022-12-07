use std::iter::Peekable;

struct Dir(Vec<Dir>, usize);

pub fn main() {
    let root = sh(&mut include_str!("../input.txt").lines().peekable());
    println!("{}", search(&root, root.1 - 40_000_000).unwrap());
}

fn sh(lines: &mut Peekable<impl Iterator<Item = &'static str>>) -> Dir {
    let (mut dirs, mut size) = (vec![], 0);
    while let Some(i) = lines.next() {
        match i {
            "$ cd .." => break,
            _ if i.starts_with("$ l") => {
                size = std::iter::from_fn(|| lines.next_if(|i| !i.starts_with('$')))
                    .filter(|i| !i.starts_with('d'))
                    .map(|i| i.split(' ').next().unwrap().parse::<usize>().unwrap())
                    .sum()
            }
            _ => dirs.push(sh(lines)),
        }
    }
    size += dirs.iter().map(|d| d.1).sum::<usize>();
    Dir(dirs, size)
}

fn search(d: &Dir, min: usize) -> Option<usize> {
    d.0.iter()
        .filter(|d| d.1 >= min)
        .flat_map(|d| [Some(d.1), search(d, min)])
        .flatten()
        .min()
}

use std::iter::Peekable;

struct Dir(Vec<Dir>, u64);

pub fn main() {
    let root = sh(&mut include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .peekable());
    println!("{}", search(&root, root.1 - 40_000_000).unwrap());
}

fn sh(lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>) -> Dir {
    let (mut dirs, mut size) = (vec![], 0);
    while let Some(i) = lines.next() {
        match i {
            b"$ cd .." => break,
            _ if &i[0..3] == b"$ l" => {
                size = std::iter::from_fn(|| lines.next_if(|i| i[0] != b'$'))
                    .filter(|i| i[0] != b'd')
                    .filter_map(|i| atoi::atoi::<u64>(i.split(|b| b == &b' ').next().unwrap()))
                    .sum()
            }
            _ => dirs.push(sh(lines)),
        }
    }
    size += dirs.iter().map(|d| d.1).sum::<u64>();
    Dir(dirs, size)
}

fn search(d: &Dir, min: u64) -> Option<u64> {
    d.0.iter()
        .filter(|d| d.1 >= min)
        .flat_map(|d| [Some(d.1), search(d, min)])
        .flatten()
        .min()
}

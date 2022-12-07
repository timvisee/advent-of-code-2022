use std::iter::Peekable;

pub fn main() {
    let (d, mut s) = (include_bytes!("../input.txt"), 0);
    sh(&mut d.split(|b| b == &b'\n').peekable(), &mut s);
    println!("{}", s);
}

fn sh(lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>, sum: &mut u64) -> u64 {
    let mut size = 0;
    while let Some(i) = lines.next() {
        match i {
            b"$ cd .." => break,
            _ if &i[0..3] == b"$ l" => {
                size = std::iter::from_fn(|| lines.next_if(|i| i[0] != b'$'))
                    .filter(|i| i[0] != b'd')
                    .filter_map(|i| atoi::atoi::<u64>(i.split(|b| b == &b' ').next().unwrap()))
                    .sum()
            }
            _ => size += sh(lines, sum),
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}

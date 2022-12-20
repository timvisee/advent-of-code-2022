use nom::*;

const ROW: isize = 2_000_000;

pub fn main() {
    let mut ranges = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| line(l).unwrap().1)
        .filter(|(s, _, d)| s.1.abs_diff(ROW) <= *d)
        .map(|(s, _, d)| {
            let rd = d as isize - s.1.abs_diff(ROW) as isize;
            s.0 - rd..=s.0 + rd
        })
        .collect::<Vec<_>>();

    let (mut merged, mut merging) = (vec![], ranges.pop().unwrap());
    loop {
        merging = match ranges
            .iter()
            .position(|other| merging.contains(other.start()) || merging.contains(other.end()))
        {
            Some(pos) => {
                let other = ranges.remove(pos);
                *(merging.start().min(other.start()))..=*(merging.end().max(other.end()))
            }
            None => {
                merged.push(merging);
                match ranges.pop() {
                    Some(cur) => cur,
                    None => break,
                }
            }
        }
    }

    println!(
        "{}",
        merged
            .into_iter()
            .map(|r| *r.end() - *r.start())
            .sum::<isize>()
    );
}

named!(usize<&[u8], isize>, map_opt!(nom::character::complete::digit1, atoi::atoi));
named!(isize<&[u8], isize>, alt!(usize | preceded!(char!('-'), map!(call!(usize), |n| -n))));
named!(coord<&[u8], (isize, isize)>, preceded!(tag!("x="), separated_pair!(isize, tag!(", y="), isize)));
named!(line<&[u8], ((isize, isize), (isize, isize), usize)>, do_parse!(
    tag!("Sensor at ") >> s: call!(coord) >> tag!(": closest beacon is at ") >> b: call!(coord) >>
    (s, b, s.0.abs_diff(b.0) + s.1.abs_diff(b.1))
));

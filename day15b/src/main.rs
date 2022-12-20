use nom::*;

const MAX: isize = 4_000_000;

pub fn main() {
    let sens = include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .map(|l| line(l).unwrap().1)
        .collect::<Vec<_>>();

    let distress = sens
        .iter()
        .flat_map(|(s, d)| {
            (0..=*d as isize)
                .map(move |i| (s.0 + i, s.1 - *d as isize - 1 + i))
                .chain((0..=*d as isize).map(move |i| (s.0 - i, s.1 + *d as isize + 1 - i)))
                .chain((0..=*d as isize).map(move |i| (s.0 - *d as isize - 1 + i, s.1 - i)))
                .chain((0..=*d as isize).map(move |i| (s.0 + *d as isize + 1 - i, s.1 + i)))
        })
        .filter(|(x, y)| (0..MAX).contains(x) && (0..MAX).contains(y))
        .find(|(x, y)| {
            !sens
                .iter()
                .any(|(s, d)| s.0.abs_diff(*x) + s.1.abs_diff(*y) <= *d)
        })
        .unwrap();

    println!("{}", distress.0 * MAX + distress.1);
}

named!(usize<&[u8], isize>, map_opt!(nom::character::complete::digit1, atoi::atoi));
named!(isize<&[u8], isize>, alt!(usize | preceded!(char!('-'), map!(call!(usize), |n| -n))));
named!(coord<&[u8], (isize, isize)>, preceded!(tag!("x="), separated_pair!(isize, tag!(", y="), isize)));
named!(line<&[u8], ((isize, isize), usize)>, do_parse!(
    tag!("Sensor at ") >> s: call!(coord) >> tag!(": closest beacon is at ") >> b: call!(coord) >>
    (s, s.0.abs_diff(b.0) + s.1.abs_diff(b.1))
));

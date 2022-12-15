#![feature(array_windows)]

use nom::*;

const W: usize = 665;
const H: usize = 165;

pub fn main() {
    let (mut map, mut sand) = ([[false; W]; H], 0);

    let mut lowest = 0;
    include_bytes!("../input.txt")
        .split(|b| b == &b'\n')
        .filter(|b| !b.is_empty())
        .map(|l| line(l).unwrap().1)
        .for_each(|coords| {
            coords.array_windows().for_each(|[a, b]| {
                lowest = lowest.max(a.1.max(b.1));
                if a.0 == b.0 {
                    (a.1.min(b.1)..=a.1.max(b.1)).for_each(|y| map[y][a.0] = true);
                } else {
                    (a.0.min(b.0)..=a.0.max(b.0)).for_each(|x| map[a.1][x] = true);
                }
            });
        });

    while let Some((x, y)) = trace(&map, (500, 0), lowest + 2) {
        if y == 0 {
            break;
        }
        sand += 1;
        map[y][x] = true;
    }

    println!("{}", sand + 1);
}

fn trace(map: &[[bool; W]; H], (mut x, y): (usize, usize), low: usize) -> Option<(usize, usize)> {
    (y..H - 1)
        .find(|y| {
            *y == low - 1
                || [x, x - 1, x + 1]
                    .into_iter()
                    .find(|x| !map[*y + 1][*x])
                    .map(|xx| x = xx)
                    .is_none()
        })
        .map(|y| (x, y))
}

named!(usize<&[u8], usize>, map_opt!(nom::character::complete::digit1, atoi::atoi));
named!(coord<&[u8], (usize, usize)>, separated_pair!(usize, char!(','), usize));
named!(line<&[u8], Vec<(usize, usize)>>, separated_list1!(nom::bytes::complete::tag(" -> "), coord));

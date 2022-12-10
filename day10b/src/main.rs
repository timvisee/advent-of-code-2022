use std::io::{stdout, Write};

pub fn main() {
    let (mut c, mut x, mut t) = (0, 1, Vec::with_capacity(40 * 60));

    for i in include_bytes!("../input.txt").split(|b| b == &b'\n') {
        t.push((x - 1 <= c % 40 && x + 1 >= c % 40) as u8 * 3 + 32);
        c += 1;
        if &i[0..4] == b"addx" {
            t.push((x - 1 <= c % 40 && x + 1 >= c % 40) as u8 * 3 + 32);
            c += 1;
            x += atoi::atoi::<isize>(&i[5..]).unwrap() as i32;
        }
    }

    let mut stdout = stdout().lock();
    t.chunks(40).for_each(|l| {
        stdout.write_all(l).unwrap();
        stdout.write_all(b"\n").unwrap();
    });
}

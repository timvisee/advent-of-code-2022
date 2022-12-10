pub fn main() {
    let (mut c, mut x, mut s) = (1, 1, 0);

    for i in include_bytes!("../input.txt").split(|b| b == &b'\n') {
        s += ((c + 20) % 40 == 0) as i32 * c * x;
        c += 1;
        if &i[0..4] == b"addx" {
            s += ((c + 20) % 40 == 0) as i32 * c * x;
            c += 1;
            x += atoi::atoi::<isize>(&i[5..]).unwrap() as i32;
        }
    }

    println!("{}", s);
}

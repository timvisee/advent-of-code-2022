pub fn main() {
    let d = include_bytes!("../input.txt");

    let mut w = 0;
    'main: loop {
        let mut seen = 0u32;
        for i in (1..=14).rev() {
            let mask = 1 << d[w + i] - b'a';
            if seen & mask == mask {
                w += i;
                continue 'main;
            }
            seen |= mask;
        }
        break;
    }

    println!("{}", w + 15);
}

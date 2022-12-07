pub fn main() {
    let d = include_bytes!("../input.txt");

    let mut w = 0;
    'main: loop {
        let mut seen = 0u32;
        for i in (0..=13).rev() {
            let mask = 1 << d[w + i] - b'a';
            if seen & mask == mask {
                w += i + 1;
                continue 'main;
            }
            seen |= mask;
        }
        break;
    }

    println!("{}", w + 14);
}

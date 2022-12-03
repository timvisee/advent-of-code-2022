pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .map(|l| l.split_at(l.len() / 2))
            .map(|(a, b)| b
                .iter()
                .filter(|b| a.contains(b))
                .map(|b| if *b >= b'a' {
                    (b - b'a') as i16 + 1
                } else {
                    (b - b'A') as i16 + 27
                })
                .next()
                .unwrap())
            .sum::<i16>(),
    );
}

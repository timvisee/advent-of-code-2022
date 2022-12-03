pub fn main() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|set| set[0]
                .iter()
                .find(|b| set[1].contains(b) && set[2].contains(b))
                .unwrap())
            .map(|b| match b {
                b'a'..=b'z' => (b - b'a') as i16 + 1,
                b'A'..=b'Z' => (b - b'A') as i16 + 27,
                _ => unreachable!(),
            })
            .sum::<i16>(),
    );
}

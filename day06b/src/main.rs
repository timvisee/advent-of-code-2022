pub fn main() {
    let d = include_bytes!("../input.txt");

    let mut w = 0;
    'main: loop {
        let mut seen = [false; 26];
        for i in (1..14).rev() {
            let b = &mut seen[(d[w + i] - b'a') as usize];
            if *b {
                w += i + 1;
                continue 'main;
            }
            *b = true;
        }
        break;
    }

    println!("{}", w + 14);
}

pub fn main() {
    let d = include_bytes!("../input.txt");
    let s = d.iter().position(|b| b == &b'\n').unwrap();
    let g: Vec<&[u8]> = d.split(|b| b == &b'\n').filter(|l| !l.is_empty()).collect();
    let mut seen = std::collections::HashSet::new();

    for i in 1..s - 1 {
        seen.extend(
            (1..s - 1)
                .scan(g[0][i], |max, y| match (*max, g[y][i]) {
                    (m, n) if n > m => {
                        *max = n;
                        Some(Some(i + y * s))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
        seen.extend(
            (1..s - 1)
                .scan(g[s - 1][i], |max, y| match (*max, g[s - y][i]) {
                    (m, n) if n > m => {
                        *max = n;
                        Some(Some(i + (s - y) * s))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
        seen.extend(
            (1..s - 1)
                .scan(g[i][0], |max, x| match (*max, g[i][x]) {
                    (m, n) if n > m => {
                        *max = n;
                        Some(Some(x + i * s))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
        seen.extend(
            (1..s - 1)
                .scan(g[i][s - 1], |max, x| match (*max, g[i][s - x]) {
                    (m, n) if n > m => {
                        *max = n;
                        Some(Some(s - x + i * s))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
    }

    println!("{}", seen.len() + ((s - 1) * 4));
}

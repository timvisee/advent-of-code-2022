use nom::*;
use std::cmp::Ordering;

pub fn main() {
    let mut packets: Vec<Item> = include_str!("../input.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| item(l.as_bytes()).unwrap().1)
        .collect();
    let first = Item::L(vec![Item::L(vec![Item::I(2)])]);
    let second = Item::L(vec![Item::L(vec![Item::I(6)])]);
    packets.extend([first.clone(), second.clone()]);
    packets.sort_unstable();
    println!(
        "{}",
        (packets.iter().position(|i| i == &first).unwrap() + 1)
            * (packets.iter().position(|i| i == &second).unwrap() + 1)
    );
}

#[derive(Clone, PartialEq, Eq)]
enum Item {
    I(u8),
    L(Vec<Item>),
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::I(a), Item::I(b)) => a.cmp(b),
            (Item::L(a), Item::L(b)) => match a.iter().cmp(b) {
                r if r != Ordering::Equal => r,
                _ => a.len().cmp(&b.len()),
            },
            (Item::I(_), Item::L(b)) if b.len() == 1 => self.cmp(&b[0]),
            (Item::I(a), Item::L(_)) => Item::L(vec![Item::I(*a)]).cmp(other),
            (Item::L(_), Item::I(_)) => other.cmp(self).reverse(),
        }
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

named!(item<&[u8], Item>, alt!(map!(list, Item::L) | map!(num, Item::I)));
named!(num<&[u8], u8>, map_opt!(nom::character::complete::digit1, atoi::atoi));
named!(list<&[u8], Vec<Item>>, delimited!(char!('['), separated_list0!(char!(','), item), char!(']')));
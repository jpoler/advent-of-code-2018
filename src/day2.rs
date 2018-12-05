use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn box_id_list() -> Vec<String> {
    let input = File::open("inputs/box_ids").unwrap();
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .unwrap()
}

fn checksum(v: &Vec<String>) -> i64 {
    let initial: (i64, i64) = (0, 0);
    let (twos, threes) = v
        .iter()
        .fold(initial, |state: (i64, i64), s: &String| -> (i64, i64) {
            let (mut twos, mut threes) = state;
            let mut counts: HashMap<char, u32> = HashMap::new();
            for ch in s.chars() {
                let counter = counts.entry(ch).or_insert(0);
                *counter += 1;
            }
            if let Some(_) = counts.values().find(|&&x| x == 2) {
                twos += 1;
            }
            if let Some(_) = counts.values().find(|&&x| x == 3) {
                threes += 1;
            }
            (twos, threes)
        });

    twos * threes
}

#[derive(Debug)]
enum Trie {
    Node { children: HashMap<char, Box<Trie>> },
    Terminal,
}

use self::Trie::{Node, Terminal};

impl Trie {
    fn new_node() -> Trie {
        let children: HashMap<char, Box<Trie>> = HashMap::new();
        Node { children }
    }

    fn insert(&mut self, s: String) {
        match self {
            Terminal => unreachable!(),
            Node { children } => {
                let n = s.chars().count();
                if n == 0 {
                    // currently we could get into trouble if we end up
                    // inserting a word that had any of it's prefixes previously
                    // inserted.
                    //
                    // Fortunately, all of the ids are the same length
                    // so it can be ignored. The borrow checker is being a
                    // prima donna so I'm moving on.
                    unreachable!("this shouldn't happen")
                } else if n == 1 {
                    let ch = s.chars().next().unwrap();
                    children.entry(ch).or_insert_with(|| Box::new(Terminal));
                } else {
                    let mut chars = s.chars();
                    let ch = chars.next().unwrap();
                    let entry = children
                        .entry(ch)
                        .or_insert_with(|| Box::new(Trie::new_node()));
                    entry.insert(chars.collect())
                }
            }
        }
    }

    fn prefix(&self, s: String) -> String {
        match self {
            Terminal => String::new(),
            Node { children } => {
                let mut chars = s.chars();
                let ch = if let Some(ch) = chars.next() {
                    ch
                } else {
                    return String::new();
                };
                if let Some(trie) = children.get(&ch) {
                    let rest = trie.prefix(chars.collect());
                    return vec![ch].into_iter().chain(rest.chars()).collect();
                } else {
                    String::new()
                }
            }
        }
    }

    // given a string with one wildcard character, find it's pair if possible
    fn find_continuation(&self, s: String) -> Option<String> {
        match self {
            Terminal => Some(String::new()),
            Node { children } => {
                let mut chars = s.chars();
                let ch = chars.next()?;
                if ch == '%' {
                    let s: String = chars.collect();
                    for (ch, trie) in children.iter() {
                        if let Some(rest) = trie.find_continuation(s.clone()) {
                            return Some(vec![*ch].into_iter().chain(rest.chars()).collect());
                        }
                    }
                    return None;
                } else {
                    let trie = children.get(&ch)?;
                    let rest = trie.find_continuation(chars.collect())?;
                    return Some(vec![ch].into_iter().chain(rest.chars()).collect());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_two_part_one_test() {
        let id_list = vec![
            String::from("aaabbrrr"),
            String::from("qq"),
            String::from("lll"),
        ];
        assert_eq!(checksum(&id_list), 4);
    }

    #[test]
    fn day_two_part_one() {
        let id_list = box_id_list();
        let cksum = checksum(&id_list);

        println!("checksum: {}", cksum);
    }

    #[test]
    fn trie() {
        let mut root = Trie::new_node();
        root.insert(String::from("foo"));
        root.insert(String::from("bar"));
        root.insert(String::from("fog"));
        root.insert(String::from("bob"));

        assert_eq!(root.prefix("foo".into()), String::from("foo"));
        assert_eq!(root.prefix("quux".into()), String::new());
    }

    #[test]
    fn day_two_part_two() {
        let id_list = box_id_list();
        let mut forward_trie = Trie::new_node();
        let mut reverse_trie = Trie::new_node();

        for forward_id in id_list {
            let id_len = forward_id.chars().count();
            let reverse_id: String = forward_id.chars().rev().collect();
            let mut forward_prefix = forward_trie.prefix(forward_id.clone());
            let mut reverse_prefix = reverse_trie.prefix(reverse_id.clone());
            if forward_prefix.len() + reverse_prefix.len() > id_len - 1 {
                // this is sort-of a heuristic, but the longer prefix is much
                // more likely to be a true prefix, where the shorter one could
                // have come from a different word. I'll need to find a more
                // deterministic way to solve this.
                if forward_prefix.len() > reverse_prefix.len() {
                    reverse_prefix.pop();
                } else {
                    forward_prefix.pop();
                }
                let mut wildcard: Vec<char> = forward_prefix.chars().collect();
                wildcard.push('%');
                wildcard.extend(reverse_prefix.chars().rev());
                let wildcard: String = wildcard.into_iter().collect();

                if let Some(other) = forward_trie.find_continuation(wildcard.clone()) {
                    println!(
                        "found it! wildcard: {} ({}, {})",
                        wildcard, forward_id, other
                    );
                }
            }

            forward_trie.insert(forward_id);
            reverse_trie.insert(reverse_id);
        }
    }
}

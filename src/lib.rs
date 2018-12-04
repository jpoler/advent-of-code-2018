use std::fs::File;
use std::io::{BufRead, BufReader};

fn frequency_list() -> Vec<i64> {
    let input = File::open("input").unwrap();
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_one_part_one() {
        let list = frequency_list();
        let iter = list.iter();
        let total: i64 = iter.sum();
        println!("total: {}", total);
    }

    #[test]
    fn day_one_part_two() {
        use std::collections::HashSet;
        let mut set: HashSet<i64> = HashSet::new();
        let list = frequency_list();
        let mut iter = list
            .iter()
            .cycle()
            .scan(0, |state, &x| {
                *state = *state + x;
                Some(*state)
            }).skip_while(move |&x| set.insert(x));
        if let Some(first) = iter.next() {
            println!("found first repeated: {}", first);
        } else {
            panic!("no repeated frequency found");
        }
    }
}

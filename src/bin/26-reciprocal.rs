#[macro_use]
extern crate cch_utils;

use std::collections::HashMap;

fn reciprocal_length(div: usize) -> usize {
    let mut states: HashMap<(usize, usize), usize> = HashMap::new();
    let mut nom: usize = 1;

    for i in 0.. {
        if nom == 0 {
            break;
        }
        while nom / div == 0 {
            nom *= 10;
        }
        let pair = (nom, div);

        if let Some(&prev) = states.get(&pair) {
            return i - prev;
        }
        states.insert(pair, i);
        nom = nom - div * (nom / div);
    }

    0
}

fn main() {
    answer!(
        "{}",
        (1..1001).max_by_key(|&i| reciprocal_length(i)).unwrap()
    );
}

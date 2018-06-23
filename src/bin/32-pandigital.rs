#[macro_use]
extern crate cch_utils;

use std::collections::HashMap;

fn is_pandigital(a: usize, b: usize, c: usize) -> bool {
    let text = format!("{}{}{}", a, b, c);
    if text.chars().filter(|&n| n == '0').count() != 0 {
        return false;
    }

    for c in "123456789".chars() {
        if text.chars().filter(|&n| n == c).count() != 1 {
            return false;
        }
    }
    true
}

#[test]
fn test_is_pandigital() {
    assert!(is_pandigital(39, 186, 7254));
}

fn main() {
    let mut products = HashMap::new();

    for a in 1..2000 {
        for b in 1..2000 {
            let prod = a * b;
            let pan = products.entry(prod).or_insert(false);
            if *pan {
                continue;
            }
            *pan = is_pandigital(a, b, prod);
        }
    }

    answer!(
        "{}",
        products
            .iter()
            .filter_map(|(&k, &v)| if v { Some(k) } else { None })
            .sum::<usize>()
    );
}

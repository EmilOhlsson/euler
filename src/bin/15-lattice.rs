#[macro_use]
extern crate cch_utils;

use cch_utils::graph::{count_paths, Vertex};
use std::rc::Rc;

#[derive(Hash, Eq, PartialEq, Debug)]
struct State {
    row: usize,
    col: usize,
}

impl Vertex for State {
    fn neighbors(&self) -> Vec<Rc<Self>> {
        let mut result = Vec::new();

        if self.row < 20 {
            result.push(Rc::new(State {
                row: self.row + 1,
                col: self.col,
            }));
        }
        if self.col < 20 {
            result.push(Rc::new(State {
                row: self.row,
                col: self.col + 1,
            }));
        }

        result
    }

    fn distance(&self, _other: &Self) -> usize {
        0
    }
}

fn main() {
    answer!("{}", count_paths(Rc::new(State { row: 0, col: 0 })));
}

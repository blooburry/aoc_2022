extern crate tuple;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use tuple::*;

pub struct Node {
    pub position: T2<i32, i32>,
    pub child: Option<Box<Node>>,
}

impl Node {
    pub fn move_node(&mut self, d: T2<i32, i32>, visited: &mut HashSet<(i32, i32)>) {

        // figure out if you need to recurse
        if self.child.is_some(){
            let (relative_y, relative_x) =
                (self.position.0 - self.child.as_ref().unwrap().position.0,
                self.position.1 - self.child.as_ref().unwrap().position.1);
            let res_vector = (relative_y + d.0, relative_x + d.1);

            // ah, you do need to recurse
            if res_vector.0.abs() == 2 || res_vector.1.abs() == 2 {
                let res_vector = T2(if res_vector.0 == 2 { res_vector.0 - 1 } else if res_vector.0 == -2 { res_vector.0 + 1 } else { res_vector.0 },
                                    if res_vector.1 == 2 { res_vector.1 - 1 } else if res_vector.1 == -2 { res_vector.1 + 1 } else { res_vector.1 });
                self.child.as_mut().unwrap().move_node(res_vector, visited);
            }
        }

        // move yo ass
        self.position += d;

        // are you the tail?
        if self.child.is_none() { visited.insert(<(i32, i32)>::try_from(self.position).unwrap()); }
    }
    pub fn new(position: T2<i32, i32>, child: Option<Box<Node>>) -> Self {
        Self { position, child }
    }
}

#![allow(unused)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod generics;
mod tests;

/// implements a boolean `and` gate thinking as input two bits and returns
/// bit as output.
pub fn and(a: u8, b: u8) -> u8 {
    match (a,b) {
        (1,1)=>1,
        _ => 0
    }
}

pub fn xor(a: u8, b: u8) -> u8 {
    match (a,b) {
        (1,0)|(0,1) => 1,
        _ => 0
    }
}




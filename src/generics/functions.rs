use std::fmt::{Debug, Display};

pub fn give_me<T: Display>(value: T) {
    println!("generic type value {}", value);
}

struct GenericStruct<T>(T);

struct Container<T> {
    item: T,
}


enum Transmission<T> {
    Signal(T),
    NoSignal,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

impl Container<u32> {
    fn sum(item: u32) -> Self {
        Container {
            item
        }
    }
}

pub fn test_generic() {
    let con = Container::sum(45);
    let a = Vec::<i32>::new();

    let num_from_string =str::parse::<u8>("32").unwrap();
    println!("parsed number {}",num_from_string)
}


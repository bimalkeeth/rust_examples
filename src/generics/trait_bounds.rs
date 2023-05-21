use std::fmt::{Display, format};

pub trait Eat{
    fn eat(&self){
        println!("eat");
    }
}

pub trait Code{
    fn code(&self){
        println!("code")
    }
}

pub trait Sleep{
    fn sleep(&self){
        println!("sleep")
    }
}

pub trait Programmer : Eat + Code +Sleep{
    fn animate(&self){
        self.eat();
        self.code();
        self.sleep();

        println!("repeat");
    }
}

pub struct Bob;

impl Eat for Bob {}

impl Code for Bob {}

impl Sleep for Bob {}

impl Programmer for Bob{}


pub fn lazy_addr(a:u32,b:u32)->impl Fn()->u32{
    move|| a + b
}

pub fn surrounds_with_braces(val:impl Display)-> impl Display{
    format!("{{{}}}",val)
}
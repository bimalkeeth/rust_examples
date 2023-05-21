use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Default,Debug,PartialEq,Copy, Clone)]
pub struct Complex<T>{
    pub re:T,
    pub im:T
}

impl <T> Complex<T>{
    pub fn new(re:T,im:T)->Self{
       return  Complex{re,im}
    }
}

impl <T:Add<T,Output=T>>Add for Complex<T>{
    type Output = Complex<T>;

    fn add(self, rhs: Complex<T>) -> Self::Output {
       return Complex{
           re:self.re + rhs.re,
           im:self.im + rhs.im
       }
    }
}


impl<T>From<(T,T)> for Complex<T>{
    fn from(value:(T,T)) -> Complex<T> {
       Complex{re:value.0,im:value.1}
    }
}

impl <T:Display> Display for Complex<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} + {}i",self.re,self.im)
    }
}

#[derive(Debug)]
struct Foo(u32);


pub fn ownership_test(){
    let foo =Foo(2049);
    let bar =&foo;

    println!("Foo is {:?}",foo);
    println!("Bar is {:?}",bar)
}


fn takes_the_n(n:u8){

}

fn takes_the_s(s:&mut String){
  s.push_str(" dddd")
}

pub fn ownership_in_action(){
    let n = 5;
    let mut s =String::from("string");

    takes_the_n(n);
    takes_the_s(&mut s);


    println!("n is {}",n);
    print!("s is {}",s);

}

#[derive(Debug)]
enum Food{
    Cake,
    Pizza,
    Salad
}

#[derive(Debug)]
struct Bag{
    food:Food
}


pub fn match_expression(){
    let bag =Bag{food:Food::Cake};

    match &bag.food {
        Food::Cake => println!("i got a cake"),
        a =>println!("i got {:?}",a)
    }

    print!("{:?}",bag);
}

struct Item(u32);

impl Item{
    fn new()->Self{
        Item(1024)
    }

    fn take_item(&self){

    }
}

pub fn check_impl_moved(){
    let it = Item::new();
    it.take_item();

    println!("{}",it.0)
}


pub fn mutable_borrow_check(){
    let mut a = String::from("owned string");
    let a_ref = &mut a;

    a_ref.push('!');

    println!("{}",&a);

}
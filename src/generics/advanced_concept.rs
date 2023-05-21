use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::mem;
use serde_derive::{Deserialize, Serialize};

pub fn advance_snippet() {
    let precompute = {
        let a = (-34i64).abs();
        let b = 345i64.pow(3);

        let c = 3;

        a+b+c

    };

    println!("{}",precompute);

    let result_msg="donea";

    let result = if result_msg=="done"{
        6
    }else{
        7
    };

    println!("{}",result);

}

struct Container{
    items_count:u32
}

enum Matcher {
    Item(u64),
    Empty }


fn increment_item(Container{mut items_count}: &mut Container){
    items_count+=1
}

fn calculate_cost(Container{items_count}:&Container)->u32{
    let rate =67;
    rate * items_count
}

pub fn test_destruction(){
    let mut container=Container{
        items_count:10
    };

    increment_item(&mut container);
    let total_cost = calculate_cost(&container);

    print!("total cost {}",total_cost);

    let mut item = Matcher::Item(56);
    if let Matcher::Item(it) = item{
        println!("{}",it)
    }else  {
        println!("failed")
    }

    let foo = "Foo";
    let bar = "Bar";
    let baz = foo.to_string() + bar;

}

pub fn get_open_file(){
    let file =File::open("foo.txt").unwrap();
    let bytes:Vec<Result<u8,_>> = file.bytes().collect();
}

pub struct ParsedPlayLoad<T>{
    inner:T
}

pub struct ParserError<E>{
    inner:E
}

pub fn parse_payload<T,E>(stream:&[u8])->Result<ParsedPlayLoad<T>,ParserError<E>>{
    unimplemented!()
}

const fn read_header(a:&[u8])->(u8,u8,u8,u8){
    (a[0],a[1],a[2],a[3])
}


const FILE_HEADER:(u8,u8,u8,u8)=read_header(include_bytes!("./smart_pointers.rs"));

pub fn show_const(){
    println!("{:?}",FILE_HEADER)
}


struct Primes{
    limit:usize
}

struct PrimesIter {
    index: i32,
    computed: Vec<bool>
}

impl Primes{
    fn iter(&self)->PrimesIter{
        PrimesIter{
            index:2,
            computed:computed_primes(self.limit)
        }
    }

    fn new(limit:usize)->Primes{
        Primes{limit}
    }
}

fn computed_primes(limit: usize) -> Vec<bool> {
    let mut sieve = vec![true;limit];
    let mut  m = 2;

    while m * m < limit {
        if sieve[m]{
            for i in (m *2..limit).step_by(m){
                sieve[i]=false
            }
        }

        m+=1
    }
    sieve
}


impl Iterator for PrimesIter{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop{
            self.index +=1;
            if self.index > (self.computed.len() - 1) as i32 {
                return None
            }else if self.computed[self.index as usize]{
               return Some(self.index as usize)
            }else{
                continue
            }
        }
    }
}

pub  fn get_primes(){
    let primes = Primes::new(100);

    for i in primes.iter(){
        println!("{}",i)
    }
}


trait Driver{
  fn drive(&self){
      println!("drivers driving!")
  }
}

struct MyCar;

impl MyCar{
    fn drive(&self){
        println!("i am  driving!")
    }
}

impl Driver for MyCar {


}

pub fn check_trait_func(){
    let car =MyCar;
    car.drive();
    println!("{}", mem::size_of_val(&RefCell::new(4)))
}

#[derive(Debug,Serialize,Deserialize)]
struct Foo{
    a:String,
    b:u64
}

impl Foo {
    fn new(a:&str,b:u64)->Self{
        Self{
            a:a.to_string(),
            b
        }
    }
}


pub fn serialize_test(){
    let foo_json = serde_json::
     to_string(&Foo::
     new("it is that simple",101)).unwrap();

    println!("{:?}",foo_json);

    let foo_value:Foo=serde_json::from_str(&foo_json).unwrap();
    println!("{:?}",foo_value)
}
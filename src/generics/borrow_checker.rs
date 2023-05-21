use std::rc::Rc;

fn take_the_n(n: &mut u8) {
    *n += 2;
}

fn take_the_s(s: &mut String) {
    s.push_str("ing")
}

pub fn borrowing_action() {
    let mut n = 5;
    let mut s = String::from("Borrow");

    take_the_n(&mut n);
    take_the_s(&mut s);

    println!("n changed to {}", n);
    println!("s changed to {}", s)
}

#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad,
}

#[derive(Debug)]
struct Bag {
    food: Food,
}

pub fn match_borrow_check() {
    let bag = Bag { food: Food::Cake };
    match bag.food{
        Food::Cake => println!("i got cake"),
        ref a => println!("i got {:?}",a)
    }

    println!("{:?}",bag)
}

fn get_borrowed_value_box()->Box<u8>{
    let x =5;

    Box::new(x)
}

fn get_borrowed_value_rc()->Rc<u8>{
    let x =5;

    Rc::new(x)
}


pub fn return_reference(){
    let val =get_borrowed_value_box();

    println!("value of return box : {}",*val);


    let rc_val=get_borrowed_value_rc();
    println!("value of return rc : {}",*rc_val);
}

struct SomeRef<'a ,T>{
    part:&'a T
}

pub fn some_ref_action(){
    let a =SomeRef{part:&43};
}



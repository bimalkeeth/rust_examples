use std::cell::{Cell, RefCell};
use std::ops::DerefMut;
use std::rc::{Rc, Weak};

struct Character{
    name:String
}

impl Drop for Character{
    fn drop(&mut self) {
        println!("{} went away",self.name)
    }
}


pub fn drop_in_action(){
    let steve=Character{
        name:"Steve".into()
    };

    let john=Character{
        name:"John".into()
    };
}


fn box_ref<T>(b:T)->Box<T>{
    let a= b;

    Box::new(a)
}

struct  Foo;

pub fn show_box_action(){
    let box_one =Box::new(Foo);
    let unboxed_one:Foo = *box_one;

    let s=box_ref(unboxed_one);

    let a = Nodex { data: 33, next: None };

}

struct Nodex{
    data:u32,
    next:Option<Box<Nodex>>
}

#[derive(Debug)]
struct Node<T>{
    next:Option<Rc<Node<T>>>,
    data:T
}

#[derive(Debug)]
struct LinkedList<T>{
    head:Option<Rc<Node<T>>>
}

impl <T> LinkedList<T>{
    fn new()->Self{
        LinkedList{head:None}
    }

    fn append(&self,data_val:T)->Self{
        LinkedList{
            head:Some(Rc::new(Node{
                data:data_val,
                next:self.head.clone()
            }))
        }
    }
}

pub fn show_rc_linked_list(){
    let list_of_nums = LinkedList::new().
        append(1).
        append(2);

    println!("nums: {:?}",list_of_nums);
    let list_of_stars = LinkedList::new().append("Foo").append("Bar");
    println!("stars:{:?}",list_of_stars)
}


#[derive(Debug)]
struct LinkList<T>{
    head:Option<Rc<LinkListNode<T>>>
}

#[derive(Debug)]
struct LinkListNode<T>{
    next:Option<Rc<LinkListNode<T>>>,
    prev:RefCell<Option<Weak<LinkListNode<T>>>>,
    data:T
}

impl <T>LinkList<T>{
    fn new()->Self{
        LinkList{head:None}
    }

    fn append(&mut self,data_val:T)->Self{
        let new_node = Rc::new(LinkListNode{
            data:data_val,
            next:self.head.clone(),
            prev:RefCell::new(None)
        });

        match self.head.clone(){
            Some(node)=>{
                let mut prev=node.prev.borrow_mut();
                *prev=Some(Rc::downgrade(&new_node));
            },

            None=>{

            }
        }

        LinkList{
            head:Some(new_node)
        }
    }

}

pub fn process_list(){
    let list_of_nums=LinkList::new().
        append(1).
        append(2).
        append(3);
    println!("nums: {:?}",list_of_nums)
}
#[derive(Debug)]
struct Bag{
    item:Box<u32>
}


pub fn interior_mutability(){
    let mut bag = Cell::new(Bag{item:Box::new(1)});
    let hand1 = &bag;
    let hand2 = &bag;

    hand1.set(Bag{item:Box::new(2)});
    hand2.set(Bag{item:Box::new(2)});


}

pub fn borrow_with_refcell(){
    let bag = RefCell::new(Bag{item:Box::new(1)});
    let hand1 =&bag;
    let hand2 = &bag;

    {
        *hand1.borrow_mut() = Bag { item: Box::new(2) };
        *hand2.borrow_mut() = Bag { item: Box::new(3) };

        let borrow_val = hand2.borrow();

        println!("value : {:?}", borrow_val)
    }
    //within separate scope we do do mutable borrow again but in same scope.
    {
        *hand1.borrow_mut() = Bag { item: Box::new(4) };
        *hand2.borrow_mut() = Bag { item: Box::new(5) };

        let borrow_val = hand2.borrow();

        println!("value : {:?}", borrow_val)
    }

}


struct Point{
    x:u8,
    y:u8,
    cached_sum:Cell<Option<u8>>
}

impl Point {
    fn sum(&self)->u8{
        match self.cached_sum.get() {
            Some(sum)=>{
                println!("got from cache :{}",sum);
                sum
            },
            None=>{
                let new_sum=self.x+ self.y;
                self.cached_sum.set(Some(new_sum));
                println!("set cache :{}",new_sum);

                new_sum
            }
        }
    }
}

pub fn pointer_test_cell(){
    let p = Point{x:8,y:9,cached_sum:Cell::new(None)};
    println!("summed result:{}",p.sum());
    println!("summed result:{}",p.sum())
}
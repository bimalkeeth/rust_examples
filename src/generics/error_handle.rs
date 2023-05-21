
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{File, read_to_string};
use std::io::Read;
use std::panic;
use std::path::Path;
use std::string::FromUtf8Error;
use crate::generics::trait_bounds::Bob;

pub fn handle_result() {
    let path = Path::new("data.txt");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("error occurred {}", err)
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    println!("message {}", s)
}

fn get_nth(items: &Vec<usize>, nth: usize) -> Option<usize> {
    if nth < items.len() {
        Some(items[nth])
    } else {
        None
    }
}

fn double(val: usize) -> usize {
    val * val
}

pub fn get_combinator() {
    let items = vec![7, 6, 4, 3, 5, 3, 10, 3, 2, 4];
    println!("{}", items.len());

    let doubled = get_nth(&items, 4).map(double);

    let doubled2 = get_nth(&items, 4).and_then(|a| {
        return Some(a * a);
    });

    println!("{:?}", doubled);
    println!("{:?}", doubled2);
}

fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(str)=>str.to_uppercase(),
        Err(err)=>return Err(err)
    };

    println!("conversation succeeded {}",ret);

    Ok(ret)
}

pub fn show_error(){
    let invalid_str=str_upper_match(vec![197,198]);
    println!("{:?}",invalid_str)
}

fn str_concise(str:Vec<u8>)->Result<String,FromUtf8Error>{
    let ret =String::from_utf8(str).map(|s|s.to_uppercase())?;

    println!("conversation succeeded {}",ret);

    Ok(ret)
}


pub fn show_error_2(){
    let valid_str =str_concise(vec![121,97,89]);

    println!("{:?}",valid_str)
}

pub fn panic_unwind(){
    panic::catch_unwind(||{
        panic!("panicking")
    }).ok();

    println!("suvived the panic")
}

#[derive(Debug)]
pub enum ParseError{
    Malformed,
    Empty
}

#[derive(Debug)]
pub struct ReadErr{
    pub child_err:Box<dyn Error>
}

impl Display for ReadErr{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"failed read to do file")
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"todo list parsing failed")
    }
}

impl Error for ReadErr {
    fn description(&self) -> &str {
        "to do list read failed"
    }

    fn cause(&self) -> Option<&dyn Error> {
       return Some(&*self.child_err)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "todo list parser failed"
    }

    fn cause(&self) -> Option<&dyn Error> {
        return None
    }
}

#[derive(Debug)]
pub struct TodoList{
    tasks:Vec<String>
}

impl TodoList {

    pub fn get_todos<P>(path:P)->Result<TodoList,Box<dyn Error>> where P:AsRef<Path>{
        let read_todos:Result<String,Box<dyn Error>>=read_todos(path);

        let parsed_todos=parse_todos(&read_todos?)?;
        Ok(parsed_todos)
    }
}

fn parse_todos(todo_str: &str) -> Result<TodoList,Box<dyn Error>>  {
    let mut tasks:Vec<String>=vec![];

    for i in todo_str.lines(){
        tasks.push(i.to_string());
    }

    if tasks.is_empty(){
        Err(ParseError::Malformed.into())
    }else {
        Ok(TodoList{tasks})
    }
}

fn read_todos<P>(path: P) -> Result<String, Box<dyn Error>> where P: AsRef<Path> {
    let raw_todos=read_to_string(path).map_err(|e| ReadErr{
        child_err:Box::new(e)
    })?;

    Ok(raw_todos)
}

pub fn load_file_error(){
    let todos=TodoList::get_todos("todo.txt");
    match todos {
        Ok(list)=>println!("{:?}",list),
        Err(err)=>{
            println!("{}",err.to_string());
            println!("{:?}",err)
        }
    }
}



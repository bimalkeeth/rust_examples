#[derive(Debug)]
struct Number<'a> {
    num:&'a u8
}

impl <'a> Number<'a>{
    fn get_num(&self)-> &'a u8{
      return self.num
    }

    fn set_num(&mut self,new_number:&'a u8){
        self.num=new_number
    }
}

pub fn get_lifetime_action(){
    let a =10;
    let mut num =Number{num:&a};
    num.set_num(&23);
    println!("{:?}",num.get_num())
}

struct Decoder<'a,'b,S,R>{
    schema:&'a S,
    reader:&'b R
}

impl <'a,'b,S,R>Decoder<'a,'b,S,R> where 'a:'b{

}

pub fn get_related_life(){
    let a:Vec<u8>=vec![];
    let b:Vec<u8>=vec![];

    let decoder = Decoder{schema:&a,reader:&b};
}

enum Level{
    Error
}

struct Logger<'a>(&'a str,Level);

fn configure_logger<T>(_t:T) where T:Send +'static{

}

pub fn get_logger(){
    let name ="Global";
    let log1=Logger(name,Level::Error);
    configure_logger(log1)
}
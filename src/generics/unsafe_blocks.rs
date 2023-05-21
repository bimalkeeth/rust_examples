unsafe fn get_values(i: *const i32) -> i32 {
    return *i;
}

pub fn show_unsafe_result() {
    let foo = &1024 as *const i32;
    let bar = unsafe { get_values(foo) };

    print!("{}", bar);
}

struct MyType;

unsafe trait UnsafeTrait{
    unsafe fn unsafe_func(&self);

    fn safe_func(&self){
        println!("things are fine here")
    }
}

trait SafeTrait{
    unsafe fn look_before_you_call(&self);
}

unsafe impl UnsafeTrait for MyType{
    unsafe fn unsafe_func(&self) {
        println!("highly unsafe");
    }
}

impl SafeTrait for MyType{
    unsafe fn look_before_you_call(&self) {
       println!("something unsafe")
    }
}


pub fn unsafe_trait_test(){
    let my_type =MyType;

    my_type.safe_func();

    unsafe {
        my_type.look_before_you_call();
    }

}
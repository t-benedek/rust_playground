use std::fmt::Debug;
use std::ops::Deref;
use std::ops::Drop;

// With using the debug trait as a requirement for T, we enable the 
struct MyBox<T> 
where T: Debug
{
    t: T,
}

impl<T> MyBox<T> 
where T: Debug
{
    fn new(x: T) -> MyBox<T> 
    where T: Debug
    {
        MyBox{t: x}
    }
}

// implementaion of deref Trait for MyBox to be able to use the derefencing operator *
impl<T> Deref for MyBox<T> 
where T: Debug
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

// We print out when the Drop trait is called for the MyBox instance
impl<T> Drop for MyBox<T> 
where T: Debug
{
    fn drop(&mut self)  
    {
        println!("Dropping Mybox with data <{:?}>", &self.t);
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    // we can now use the dereference operator * because we implemented deref for the MyBox struct
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    
    // This is "Deref coercion" in action
    // Rust converts &MyBox<String> into &String because we implemented the deref Trait
    // Standard library implememnts deref Trait on String so that &String can be converted to %str
    hello(&m);

    // This is also possible but hard to read
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}");
}


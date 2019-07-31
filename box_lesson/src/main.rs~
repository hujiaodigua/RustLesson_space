// use crate::List::{Cons, Nil};
// 
// enum List{
//     Cons(i32, Box<List>),
//     Nil,
// }
// 
// fn main() 
// {
//     let list = Cons(1, 
//         Box::new(Cons(2, 
//             Box:new(Cons(3, 
//                 Box::new(Nil))))));
// }

// fn main()
// {
//     let x = 5;
//     let y = &x;    // y保存的是x的引用(地址)
// 
//     assert_eq!(5, x);
//     assert_eq!(5, *y);    // *解引用取出y保存的引用(地址)所代表的值
// }

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// 定义MyBox类型
struct MyBox<T>(T);
impl <T> MyBox<T>
{
    fn new(x: T) -> MyBox<T>
    {
        MyBox(x)
    }
}

fn hello(name: &str)    // &str类型的参数name
{
    println!("Hello, {}!", name);
}

// 像引用(地址&)一样使用Box<T>
fn main()
{
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

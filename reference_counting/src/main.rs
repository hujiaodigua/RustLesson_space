use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List
{
    Cons(i32 ,Rc<List>),    // Box是不行的,Rc才可以
    Nil,                    // 使用 Rc 允许一个值有多个所有者，引用计数则确保只要任何所有者依然存在其值也保持有效,Rc<T> 允许通过不可变引用来只读的在程序的多个部分共享数据
}

fn main() 
{
    // let a = Cons(5,
    //     Rc::new(Cons(10,
    //         Rc::new(Nil))));

    // let b = Cons(3, Rc::clone(a));    // clone会增加引用计数
    // let c = Cons(4, Rc::clone(a));    // 就是实际上只是引用拷贝

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    
}

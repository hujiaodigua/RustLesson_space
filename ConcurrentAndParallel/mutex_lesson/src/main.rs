use std::sync::{Mutex, Arc};
use std::thread;
use std::rc::Rc;

fn function_1()
{
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// 在线程间共享Mutex<T>
// 多线程和多所有权
// fn function_2()
// {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];
// 
//     for _ in 0..10
//     {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move ||{
//             let mut num = counter.lock().unwrap();
// 
//             *num += 1;
//         });
//         handles.push(handle);
//     }
// 
//     for handle in handles
//     {
//         handle.join().unwrap();
//     }
// 
//     println!("Result {}", *counter.lock().unwrap());
// 
// }

// 原子引用计数Arc<T>
fn function_3()
{
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 
    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}

fn main() 
{
    // function_1();
    // function_2();
    function_3();

}

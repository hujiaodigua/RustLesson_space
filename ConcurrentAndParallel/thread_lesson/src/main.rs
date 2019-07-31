use std::thread;
use std::time::Duration;

fn main() 
{
    let handle = thread::spawn(||{    // 传递一个闭包,thread::spawn的返回值类型为JoinHandle
        for i in 1..10
        {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5
    {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();    // 等待线程执行完毕
}

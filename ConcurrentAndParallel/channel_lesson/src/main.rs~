use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn function_1() 
{
    let (tx, rx) = mpsc::channel();    // mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写
                                       // 一个channel有tx与rx

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();    //  tx 移动到一个新建的线程中并发送 “hi”
    });

    let received = rx.recv().unwrap();    // 在主线程中用rx接收并打印内容 “hi”
    println!("Got: {}", received);
}

//  发送多个消息,并在每次发送后暂停一段时间
fn function_2()
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals
        {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx
    {
        println!("Got: {}", received);
    }

}

// 通过克隆发送者来创建多个生产者
fn function_3()
{
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move ||{
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals
        {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx 
    {
        println!("Got: {}", received);
    }
}

fn main()
{   // function_1();
    // function_2();
    function_3();
}

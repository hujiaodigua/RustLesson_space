fn main() 
{
    // 匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}",x , y);

    // 使用| 匹配多个模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 使用 ... 匹配值的范围
    let x = 5;
    match x {
        1 ... 5 => println!("one through five"),
    _ => println!("something else"),
    }

    // 解构结构体,通过带有模式的let语句将其分解
    let p = Point {x: 0,y: 7};

    // let Point {x: a, y: b} = p;
    // assert_eq!(0, a);
    // assert_eq!(7, b);

    match p {
        Point {x, y: 0} => println!("On the x axis at {}",x),
        Point {x: 0, y} => println!("On the y axis at {}",y),
        Point {x, y} => println!("On neither axis: ({}, {})",x,y),
    }

    // 解构枚举
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },

        Message::Move {x, y} => {
            println!("Move in the x direction {} and in the y direction {}",x, y);
        },

        Message::Write(text) => println!("Text message: {}", text),

        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}",r, g, b)
        },
    }

}

struct Point
{
    x: i32,
    y: i32,
}

enum Message
{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

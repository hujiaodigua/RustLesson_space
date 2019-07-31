/*enum IpAddrKind
{
    V4,
    V6,
}

struct IpAddr
{
    kind: IpAddrKind,
    address: String,
}

fn main()
 {
    let home = IpAddr
    {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr
    {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

}*/

/*enum IpAddr
{
    V4(String),
    V6(String),
}

fn main()
{
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}*/

enum Message
{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // 类单元结构体
struct MoveMessage
{
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

impl Message
{
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn main()
{
    let m = Message::Write(String::from("hello"));
    m.call();
}
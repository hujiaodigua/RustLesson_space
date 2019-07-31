fn main() 
{
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop()    // 只要stack.pop()返回Some就打印出其值
    {
        println!("{}",top);
    }
    // println!("Hello, world!");
}

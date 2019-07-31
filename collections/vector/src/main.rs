fn main() {
    //let v: Vec<i32> = Vec::new();    // 这是包含类型注解的写法

    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    let third: &i32 = &v[2];                    // 使用索引获取vector中的项
    println!("The third element is {}", third );

    match v.get(2)                             // 使用get方法获取vector中的项
    {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    //println!("Hello, world!")
}

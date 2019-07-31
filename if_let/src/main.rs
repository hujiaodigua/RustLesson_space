fn main()
{
    let some_u8_value = Some(0u8);

    match some_u8_value
    {
        Some(3) => print!("three");    // match只关心当值为Some(3)时执行代码
        _ => (),
    }

    if let Some(3) = some_u8_value     // 可以认为if let是match的一个语法糖
    {
        println!("three");
    }
}

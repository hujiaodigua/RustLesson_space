fn main()
{
    let number = 3;
    if number < 5
    {
        println!("Y");
    }
    else
    {
        println!("N");
    }
    // println!("Hello, world!");

    let condition = true;
    let number =
    if condition
    {
        5
    }
    else
    {
        6
    };
    println!("The value of number is : {}", number);


    let mut counter = 0;

    let result =
    loop
    {
        println!("again!");
        counter += 1;
        if counter == 10
        {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);


    let mut number = 3;

    while number != 0
    {
        println!("{}!",number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30,40, 50];

    for element in a.iter()
    {
        println!("the value is {}", element);
    }

    for number in (1..4).rev()
    {
        println!("{}!",number);
    }
    println!("LIFTOFF!!!");

    let s1 = String::from("hello");
    let s2 = s1.clone();                //字符串这种创建后（有了自己的内存区域），是不可以直接赋值的，需要使用拷贝函数
    println!("s = {}, s2 = {}", s1, s2);

    let s = String::from("ssoo");       // s 进入作用域
    takes_ownership(s);                 // s 的值移动到函数里 ...
                                        // ... 所以到这里不再有效
    let x = 5;                          // x 进入作用域

    makes_copy(x);                      // x 应该移动函数里
                                        // 但i32是copy的，所以在后面可继续使用x
                                        // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
                                        // 所以不会有特殊操作

}

fn takes_ownership(some_string: String) // some_string进入作用域
{
    println!("{}", some_string);
} // 这里，some_string移出作用域并调用"drop"方法。占用的内存被释放

fn makes_copy(some_integer: i32)        // some_integer进入作用域
{
    println!("{}", some_integer);
} // 这里，some_integer移出作用域。不会有特殊操作

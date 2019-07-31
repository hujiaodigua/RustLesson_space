fn main() 
{
    println!("Hello, world!");

    another_fun(5);

    let x = 5;
    let y = 
    {
        let x = 3;
        x + 1
    };
    println!("x is {}, y is {}", x, y);

    let z = five();
    println!("z is {}", z);

    let q = plus_one(5);
    println!("q is {}", q);
}

fn another_fun(x: i32)
{
    println!("the value of x is: {}", x);

}

fn five() -> i32
{
    5
}

fn plus_one(x: i32) -> i32
{
    x + 1
}



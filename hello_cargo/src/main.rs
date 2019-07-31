fn main() {
    println!("Hello, world!");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_1 = (500, 6.4, 1);
    let(x, y, z) = tup_1;
    println!("The value of y is:{}", y);

    let second = tup.1;
    println!("The value of two is:{}", second);

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // type; number
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("first = {},second = {}",first,second);


}

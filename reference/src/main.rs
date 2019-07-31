fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!(" the length of '{}' is {} ", s1 , len);

    let mut s = String::from("hello");    // 可变变量s
    change(&mut s);                       // 可变引用&mut s
}

fn calculate_length(s: &String) -> usize
{
    s.len()
}

fn change(some_string: &mut String)      // 接受一个可变引用some_string: &mut String
{
    some_string.push_str(",world");
}

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String
{
    let s = String::from("hello");

    // &s                            // s已经离开作用域被丢弃了，相应的内存区域被释放掉了，不可能再去返回引用了
    s

}

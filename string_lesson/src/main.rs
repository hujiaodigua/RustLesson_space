fn main()
{
    // 两个字符串级联
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    // 多个字符串级联方式1
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;    // 还是用"+"就很落后

    // 多个字符串级联方式2
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);  // 用format!宏做级联

    for c in "नमस्ते".chars()    // chars方法会打印出字符串中的字符
    {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes()    // bytes方法会打印出原始字节的Unidoce标量值
    {
        println!("{}", b);
    }
}

fn main() 
{
    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate()    // enumerate方法适配一个迭代器来产生一个值和其在迭代器中的索引
    {
        println!("{} is at index {}", value, index);
    }

    // println!("Hello, world!");
}

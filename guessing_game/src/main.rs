use std::io;                         // 从std库中导入io类
use std::cmp::Ordering;              // 从std库中导入cmp::Ordering类
use rand::Rng;                       // 从rand库中导入Rng类

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}",secret_number);

    loop
    {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");    // 获得值输入给u32类型变量guess
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,  
        };


        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number)          
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
            {
                print!("You win!");
                break;
            }
        }
    }
}

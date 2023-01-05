use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println! 在屏幕上打印字符串的宏
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // let语句创建变量
        // Rust中,变量默认是不可变的
        // 创建一个可变变量，当前绑定到一个新的String空实例上
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust 允许用一个新值隐藏 shadow guess之前的值
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _是一个通配符值
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

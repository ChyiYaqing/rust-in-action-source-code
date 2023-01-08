fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //
    if number != 0 {
        println!("number was something other than zero");
    }

    // 使用else if处理多重条件
    // 使用过多的else if表达式会使代码显得杂乱无章，所以如果多于一个else if表达式，最好重构代码
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };
    println!("The value of number is: {number2}");
}

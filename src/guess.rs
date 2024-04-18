//io 库来自于标准库,Rust 设定了若干个会自动导入到每个程序作用域中的标准库内容，这组内容被称为 预导入（preclude） 内容
//你需要的类型不在预导入内容中，就必须使用 use 语句显式地将其引入作用域
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// 猜数字
fn guess_number() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..=100); // 32位整形
  println!("The secret number is: {secret_number}");//符号 ! 调用的是宏而不是普通函数，宏并不总是遵循与函数相同的规则
  // loop 关键字创建了一个无限循环，数字匹配后 break 跳出循环
  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
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
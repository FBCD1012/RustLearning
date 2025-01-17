//传递进入相关的输入包
use std::io;
use rand::Rng;
//此处引入对比包
use std::cmp::Ordering;

fn main() {
    //注意变量之间的差异化操作理解
    let _secret_number = rand::thread_rng().gen_range(1..=100);
    //第一种方案 编译器会告警但是不会出现无法运行的情况
    loop {
        println!("Guess what num do u want");
        //生成随机数进行详细操作(变量选择实现操作)
        println!("input your num to get it");
        //这里的形式是创建一个字符串吗？ （mut 变量可变关键字）
        let mut guess=String::new();
        //此处的表达式形式操作
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //此处构建的是字符串如何进行理解？
        let guess: u32 = match guess.trim().parse() {
            //这样可以进行类型判定吗？
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your input is {}",guess);
        match guess.cmp(&_secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            //构建合理的参数进行了
            Ordering::Equal => {
                println!("You guess the num rightly!");
                break;
            }
        }
    }
}

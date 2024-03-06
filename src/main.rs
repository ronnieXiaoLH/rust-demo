// mod number_guessing;

// fn main() {
    // println!("Hello rust");
    // number_guessing::main();
// }

use std::env;
use std::process;

use hello_cargo;

fn main() {
    // 获取命令行参数的集合，第一个值是二进制文件的名称
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);

    let config = hello_cargo::Config::new(&args).unwrap_or_else(|err| {
        // 使用标准库提供的 `eprintln!` 宏打印错误信息到标准错误流
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = hello_cargo::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}















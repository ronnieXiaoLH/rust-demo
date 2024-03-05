// mod number_guessing;

// fn main() {
    // println!("Hello rust");
    // number_guessing::main();
// }

use std::fs::File;
use std::io::{self, Read};

// 一个简单的函数，从文件中读取内容并返回一个 Result<String, io::Error>
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // 尝试打开文件，如果失败则返回 Err

    let mut content = String::new();
    file.read_to_string(&mut content)?; // 尝试读取文件内容，如果失败则返回 Err

    Ok(content) // 如果成功，则返回包含文件内容的 Result
}

// 主函数，调用上述函数并处理可能的错误
fn main() {
    let filename = "example.txt";

    match read_file_content(filename) {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}













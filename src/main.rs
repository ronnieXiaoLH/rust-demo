// mod number_guessing;

// fn main() {
    // println!("Hello rust");
    // number_guessing::main();
// }

fn main() {
  // 使用 Some 包装一个值
  let some_value: Option<i32> = Some(42);

  // 使用 None 表示空值
  let no_value: Option<i32> = None;

  // 使用模式匹配提取值
  match some_value {
      Some(value) => println!("Has value: {}", value),
      None => println!("No value"),
  }

  match no_value {
      Some(value) => println!("Has value: {}", value),
      None => println!("No value"),
  }
}














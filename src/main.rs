// mod number_guessing;

// fn main() {
    // println!("Hello rust");
    // number_guessing::main();
// }

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("{:?}", six);
//     println!("{:?}", none);
// }


fn main() {
  // 假设这是一个数字变量
  let my_number = 8;

  // 使用 if let 检查是否为偶数
  if let even_number = my_number % 2 {
      if (even_number == 0) {
          println!("{} 是偶数！", my_number);
      } else {
          println!("{} 不是偶数！", my_number);
      }
  } 
}









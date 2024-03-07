// mod number_guessing;

// fn main() {
    // println!("Hello rust");
    // number_guessing::main();
// }

// use std::env;
// use std::process;

// use hello_cargo;

// fn main() {
//     // 获取命令行参数的集合，第一个值是二进制文件的名称
//     let args: Vec<String> = env::args().collect();

//     // println!("{:?}", args);

//     let config = hello_cargo::Config::new(&args).unwrap_or_else(|err| {
//         // 使用标准库提供的 `eprintln!` 宏打印错误信息到标准错误流
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

//     if let Err(e) = hello_cargo::run(config) {
//         eprintln!("Application error: {}", e);

//         process::exit(1);
//     }
// }

// use std::thread;
// use std::time::Duration;

// struct Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T> 
//     where T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher { calculation, value: None }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// fn generate_workout(intensity: u32, random_num: u32) {
//     let mut expensive_result = Cacher::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });

//     if intensity < 6 {
//         println!("先做俯卧撑{}组", expensive_result.value(intensity));
//         println!("再做仰卧起坐{}组", expensive_result.value(intensity));
//     } else {
//         if random_num == 3 {
//             println!("周三休息")
//         } else {
//             println!("先练力量{}组", expensive_result.value(intensity));
//             println!("再跑步{}分钟", expensive_result.value(intensity) * 10);
//         }
//     }
// }

// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 5;

//     generate_workout(
//         simulated_user_specified_value,
//         simulated_random_number
//     );
// }


fn main() {
}






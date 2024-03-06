## rust 安装

windows 系统在 [rust 官网](https://www.rust-lang.org/tools/install) 下载 `RUSTUP-INIT.EXE` 执行即可。

## 设置国内的 rust 镜像源

在命令行输入：

```shell
set CARGO_REGISTRY=https://mirrors.ustc.edu.cn/crates.io-index
```

查看是否设置成功，在命令行输入：

```shell
echo %CARGO_REGISTRY%
```

## 变量和可变性

- 使用 `let` 声明的变量是不可变的，除非在变量名前加上 `mut`
- 使用 `let mut` 声明的变量，可以修改为同类型的其他值，不可以修改为不同类型的其他值
- 使用 `let mut` 声明的变量，可以使用 `let` 加相同的变量名，修改为不同类型的其他值（创建一个新的变量）
- 使用 `const` 声明常量，且必须注明值的类型，命名约定是全部字母使用大写，并使用下划线分割单词

```rs
let mut x: i32 = 1;
println!("{}", x);
x = 2;
println!("{}",x);
let x = "a";
println!("{}",x);

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## 数据类型

- 标量类型（表示单个值）：整形、浮点型、布尔型和字符。
- 复合类型（多个值合成）：元组和数组。

### 整数类型

整数是没有小数部分的数字。分为有符号（`i`）和无符号（`i`）两种，区别就是无符号可以为负数。

| **长度** | **有符号类型** | **无符号类型** |
| -------- | -------------- | -------------- |
| 8 位     | i8             | u8             |
| 16 位    | i16            | u16            |
| 32 位    | i32            | u32            |
| 64 位    | i64            | u64            |
| 128 位   | i128           | u128           |
| arch     | isize          | usize          |

`isize` 和 `usize` 类型取决于程序运行的计算机体系结构，在表中表示为“arch”：若使用 64 位架构系统则为 64 位，若使用 32 位架构系统则为 32 位。

**整形字面量**

| **数字字面量**    | **示例**    |
| ----------------- | ----------- |
| 十进制            | 98_000      |
| 十六进制          | 0xff        |
| 八进制            | Oo77        |
| 二进制            | 0b1111_0000 |
| 字节（仅限于 u8） | b'A'        |

可以按上述表格所示的任意形式来编写整形字面量。允许使用类型后缀来制定类型，例如 `57u8`。还可以使用 `_` 作为分隔符来方便读数，如 `1_000`。

```rs
let a = 57u8;
println!("{}", a); // 57

let b = 1_000;
println!("{}", b); // 1000

let c = 0xff; // 255
println!("{}", c);

let d = 0b1111_0000;
println!("{}", d); // 240
```

### 浮点类型

浮点数是带有小数点的数，浮点型有两种 `f32` 和 `f64`，浮点数都是有符号的。

### 布尔类型

rust 中的布尔类型使用 `bool` 声明，两个可能的值为 `true` 和 `false`。

### 字符类型

rust 中字符类型使用 `char` 声明，它的值使用单引号包裹。

```rs
let c: char = 'A';
```

如果你想声明一个变量为一个字符串，应该使用 `&str` 声明，它的值使用双引号包裹。

```rs
let msg: &str = "Hello, Rust!"
```

### 元组

元组是将多种类型的多个值组合到一个复合类型中，声明后，元组的长度无法改变。

通过在小括号内以逗号分隔的值列表的形式来创建元组，元组中每个位置都有一个类型，不同位置之间的类型可以不同。

**元组的解构：**

- 模式匹配
- `.`连上要访问的值的索引

```rs
let tup: (i32, f64, &str) = (500, 6.4, "Hello, Rust!");

let (x, y, z) = tup;

println!("x: {}, y: {}, z: {}", x, y, z); // x: 500, y: 6.4, z: Hello, Rust!

let a = tup.0;
let b = tup.1;
let c = tup.2;

println!("a: {}, b: {}, c: {}", a, b, c); // a: 500, b: 6.4, c: Hello, Rust!
```

### 数组类型

与元组不同，数组中每个元素的类型是相同的，数组的长度是固定的。

使用方括号编写数组的类型，其中包含每个元素的类型、分号，然后是数组中的元素数，如下：

```rs
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

如果要创建每个元素值相等的数组，可以制定初始值，后跟分号，然后在方括号中指定数组的长度，如下：

```rs
let arr = [3; 5];
```

数组在栈上分配已知固定大小的内存块，可以使用索引访问数组的元素。索引值不可越界。

```rs
let arr: [i32; 5] = [2, 4, 6, 8, 10];

println!("{}, {}", arr[0], arr[1]); // 2, 4
```

## 控制流

- if 表达式
- 循环

### 循环

rust 有三种循环：`loop`、`while` 和 `for`。

1. **loop 循环**

**loop 循环终止：** 使用 `break` 跳出循环，使用 `continue` 跳出当前循环迭代，转到下一个迭代。

```rs
let mut n = 0;

loop {
    n = n + 1;
    if n == 2 {
        continue;
    }
    println!("n: {}", n);
    if n > 3 {
        break;
    }
}
```

上述示例代码中，会打印 `n: 1`，`n: 3`，`n: 4`。当 n 的值为 2 的时候，跳出当前循环迭代，当 n 的值为 4 的时候，跳出 loop 循环。

**嵌套循环：** `break` 和 `continue` 应用于最内层的循环，如果想指定 `break` 和 `continue` 应用于某个循环，可以在循环上指定一个循环标签。如下：

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

上述示例的打印信息如下：

```
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

外层循环有一个标签 `counting_up`，它将从 0 数到 2。没有标签的内部循环从 10 向下数到 9。第一个没有指定标签的 `break` 将只退出内层循环。`break 'counting_up;` 语句将退出外层循环。

**循环返回值：** 在用于停止循环的 `breack` 表达式添加要返回的值。

```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); // The result is 20
}
```

2. **while 条件循环**

当条件为真时，执行循环；当条件为假时，终止循环。

```rs
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

上述示例的打印信息如下：

```
3!
2!
1!
LIFTOFF!!!
```

3. **for 遍历集合**

使用 `for` 循环来对一个集合中的每个元素执行一些代码。比如：

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
```

上述示例的打印信息如下：

```
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

## 所有权

rust 中的所有权规则：

- rust 中的每一个值都有一个被称为其**所有者（owner）**的变量
- 值在任一时刻有且只有一个所有者
- 当所有者（变量）离开作用域，这个值将被丢弃

字符串（`String`）类型的数据是存储在堆上的。`String` 类型的变量的值赋值给其他变量后，该变量将会失去对该值的所有权。如下：

```rs
let s1 = String::from("hello");
let s2 = s1;
println!("s1: {}", s1); // value borrowed here after move
```

上面示例代码中，变量 `s1` 的值赋值给变量 `s2` 后，`s1` 将不再有效。

如果需要深度复制 `String` 中堆上的数据，可以使用 `clone` 通用函数。如下：

```rs
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1: {}", s1);
```

对于已知大小的类型的数据是存储在栈上的，定义为这种类型的变量的值赋值给其他变量，是可以直接快速拷贝其值的。以下是一些 `Copy` 的类型：

- 所有的整形
- 布尔类型
- 字符类型（`char`）
- 元组

**引用：** 允许你使用值，但获取其值的所有权。使用 `&` 引用。如下：

```rs
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len); // The length of 'hello' is 5.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

上面示例代码中，传递 `s1` 的引用 `&s1` 给 `calculate_length` 函数，同时，在 `calculate_length` 函数定义中，获取 `&String` 而不是 `string`。

**借用：** 创建一个引用的行为就是借用。

默认是不允许修改借用的值的，如下：

```rs
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // cannot borrow `*some_string` as mutable, as it is behind a `&` reference
}
```

**可变引用：** 顾名思义，就是可改变借用值的引用。如下：

```rs
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("s: {}", s); // s: hello, world
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

可变引用由一个很大的限制：在同一时间，只能由一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败，如下：

```rs
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time

    println!("{}, {}", r1, r2);
}
```

**引用的规则：**

- 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变的引用。
- 引用必须总是有效的。

**切片 Slice 类型**

slice 允许你引用集合中一段连续的元素序列，而不引用整个集合。

1. 字符串 slice

```rs
fn main() {
    let my_string = String::from("Hello World");

    let s1 = &my_string[0..5];
    let s2 = &my_string[..5];
    let s3 = &my_string[6..11];
    let s4 = &my_string[6..];
    let s5 = &my_string[..];

    println!("s1: {}", s1); // s1: Hello
    println!("s2: {}", s2); // s2: Hello
    println!("s3: {}", s3); // s3: World
    println!("s4: {}", s4); // s4: World
    println!("s5: {}", s5); // s5: Hello World
}
```

上述示例的代码及打印的结果，可以看出字符串 slice 的使用语法。

2. 数组 slice

```rs
fn main() {
    let arr = [1, 2, 3, 4, 5];

    let a1 = &arr[0..2];
    let a2 = &arr[..2];
    let a3 = &arr[2..5];
    let a4 = &arr[2..];
    let a5 = &arr[..];

    println!("a1: {:?}", a1); // a1: [1, 2]
    println!("a2: {:?}", a2); // a2: [1, 2]
    println!("a3: {:?}", a3); // a3: [3, 4, 5]
    println!("a4: {:?}", a4); // a4: [3, 4, 5]
    println!("a5: {:?}", a5); // a5: [1, 2, 3, 4, 5]
}
```

上述示例的代码及打印的结果，可以看出数组 slice 的使用语法。

## 结构体

结构体和元组一样，每一部分可以是不同的类型。使用 `struct` 关键字定义一个结构体，并定义每一字段的名字和类型。如下：

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

**创建结构体的实例**

```rs
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername"),
    active: true,
    sign_in_count: 1,
};
```

**修改结构体实例的值**

```rs
user1.email = String::from("anotheremail@example.com");
```

**结构体更新语法**

```rs
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

上述代码中，user2 是使用结构体更新语法基于 user1 创建一个新的 `User` 实例。`..user1` 必须放在最后，以指定其余字段从 `user1` 的相应字段中获取其值。

**元组结构体**

元组结构体没有具体的字段名，只有字段的类型。每一个结构体有其自己的类型，即使不同的结构体中的字段有完全相同的类型，它们也是不同的结构体类型。

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

let Color(r, g, b) = black;

println!("r: {}, g: {}, b: {}", r, g, b);
println!("{}, {}, {}", origin.0, origin.1, origin.2);
```

上述示例代码，介绍了元组结构体通过索引取值和解构。

**方法语法**

在前面我们介绍的结构体的字段中，并没有出现方法，下面的示例就是为结构体定义方法：

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

上述示例中，使用 `impl` 关键字使函数定义于结构体 `Rectangle` 的上下文中。所有定义在 `impl` 块中的函数被称为**关联函数**。方法的第一个参数必须是一个名为 `self` 的 `Self` 类型，加上 `&` 表示只希望读取结构体的数据，并不想获取结构体的所有权。`&self` 是 `self: &Self` 的简写。

一个结构体允许拥有多个 `impl` 块，如下：

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

上述示例中，`Rectangle` 结构体有多个 `impl` 块，而且它的 `can_hold` 方法中定义了多个参数。

我们还可以定义不以 `self` 为第一参数的关联函数，使用结构体名和 `::` 语法来调用这个关联函数。如下：

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let sq = Rectangle::square(2);

dbg!(sq);
```

## 枚举

**定义枚举**

```rs
enum IpAddrKind {
    V4,
    V6,
}
```

上述代码中，定义了一个 `IpAddrKind` 枚举，`V4` 和 `V6` 是这个枚举的**成员**。

**枚举值**

```rs
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

上述代码中，使用枚举的值的格式是 `枚举命::枚举成员`。

**将数据附加到枚举成员上**

```rs
enum IpAddrKind {
    V4(String),
    V6(String),
}

let home = IpAddrKind::V4(String::from("127.0.0.1"));
let loopback = IpAddrKind::V6(String::from("::1"));
```

可以将任意类型的数据放入到枚举成员中。如下：

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

**给枚举定义方法**

枚举可以像结构体那样，使用 `impl` 来定义方法。如下：

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}
```

**Option 枚举**

在 rust 中，`Option` 枚举中有 `Some` 和 `None` 两个变体，用于处理可能存在或不存在的值的情况。它被设计用于表示一个可能包含值的容器，或者肯呢个为空的情况。

1. **Some:**

- `Some` 是 `Option` 枚举的一个变体，用于表示一个包含值的情况。
- 当使用 `Some` 时，它包装了一个可能的值，表示这个值是存在的。

```rs
let some_value: Option<i32> = Some(42);
```

上述代码中，`Some(42)` 表示包含整数 42 的 `Option`。

2. **None:**

- `None` 同样是 `Option` 枚举的一个变体，用于表示一个空值或不存在值的情况。
- 当使用 `None` 时，它表示不包含值的情况，即空。

```rs
let no_value: Option<i32> = None;
```

上述代码中，`None` 代表一个不包含值的 `Option`。

```rs
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
```

上述代码将输出：

```
Has value: 42
No value
```

## match 控制流

对于 `if`，表达式必须返回一个布尔值，对于 `match` 来说，它可以是任何类型。

```rs
fn main() {

    enum Color {
        Red,
        Green,
        Blue,
    };

    fn value_in_colors(color: Color) -> u8 {
        match color {
            Color::Red => {
                println!("RED!"); // RED!
                0
            }
            Color::Green => 1,
            Color::Blue => 2,
        }
    };

    println!("{}", value_in_colors(Color::Red)); // 0
}
```

上述代码中，`metch` 用来匹配 `Coin` 枚举的值。

匹配分支另一个有用的功能是可以绑定匹配的模式的部分值。如下：

```rs
fn main() {

    enum Color {
        Red,
        Green,
        Blue(String),
    };

    fn value_in_colors(color: Color) -> u8 {
        match color {
            Color::Red => {
                println!("RED!"); // RED!
                0
            }
            Color::Green => 1,
            Color::Blue(value) => {
                println!("value is {}", value); // value is Blue Color
                2
            },
        }
    };

    println!("{}", value_in_colors(Color::Blue(String::from("Blue Color")))); // 2
}
```

**通配模式和\_占位符**

有时候，我们没有办法穷举出 `match` 的所有分支，我们可以在最后添加一个通配分支。如下：

```rs
fn main() {
    fn matchNum(num: i32) {
        match num {
            1 => println!("It's one!"),
            2 => println!("It's two!"),
            _ => println!("It's something else!"),
        }
    };

    matchNum(1);
    matchNum(2);
    matchNum(3);
    matchNum(4);
}
```

上述示例中，将依次输出：

```
It's one!
It's two!
It's something else!
It's something else!
```

## 包和 crate

包中所包含内容的规则：

- 一个包中至多只能包含一个库 crate(library crate)
- 一个包中可以包含任意多个二进制 crate(binary crate)
- 包中至少包含一个 crate，无论是库的还是二进制的

如果我们使用 `cargo new demo` 命令创建了一个 `demo` 项目，项目的中只包含一个 `src/main.rs` 的文件，此时，项目只含有一个名为 `demo` 的二进制 crate。
如果一个项目中同时包含 `src/main.rs` 和 `src/lib.rs`，则它有两个 crate：一个库和一个二进制，且名字都和包同名。
如果将文件放在 `src/bin` 目录下，一个包可同时拥有多个二进制 crate，`src/bin` 目录下的文件都会被编译为一个独立的二进制 crate。

## 模块

**模块的概念**

在 rust 中，模块是由 `mod` 关键字开始的代码块，可以包含函数、结构体、枚举、常量和其他模块。模块可以嵌套。

```rs
// 定义一个模块
mod my_module {
    // 模块内部的内容
    pub fn my_function() {
        // 函数体
    }
}
```

**访问控制**

rust 使用 `pub` 关键字来制定项的可见性。如果一个项没有使用 `pub` 修饰，它默认是私有的，只能在同一个模块中访问。使用 `pub` 可以将项暴露给外部模块使用。

```rs
mod my_module {
    // 在模块内部定义的函数，默认是私有的
    fn private_function() {}

    // 使用 pub 将函数暴露给外部模块
    pub fn public_function() {}
}
```

**模块路径**

模块路径用于定位模块中的项。rust 使用双冒号 `::` 表示模块路径。路径可以是绝对路径或相对路径。

```rs
// 相对路径
mod my_module {
    pub fn my_function() {}
}

// 在同一模块内调用
my_module::my_function();

// 绝对路径
crate::my_module::my_function();
```

**文件系统和模块**

在 rust 中，默认情况下，会根据文件系统中的结构，将代码分成模块。每个文件都可以包含一个模块，且文件名与模块名相同。

```rs
// src/main.rs
mod my_module;

// src/my_module.rs
pub fn my_function() {
    // 函数体
}
```

**使用`mod`关键字引入其他模块**

使用 `mod` 关键字可以在一个模块中引入另一个模块。

```rs
mod my_module {
    // 引入另一个模块
    mod sub_module;

    // 使用另一个模块中的函数
    pub fn call_sub_module_function() {
        sub_module::sub_function();
    }
}

// 在另一个文件中定义被引入的模块
mod sub_module {
    pub fn sub_function() {
        // 函数体
    }
}
```

**使用`use`关键字简化路径**

`use` 关键字允许将路径引入当前作用域，从而简化代码。

```rs
mod my_module {
    // 引入另一个模块
    mod sub_module;

    // 使用 use 简化路径
    use self::sub_module::sub_function;

    // 使用被引入模块的函数
    pub fn call_sub_module_function() {
        sub_function();
    }
}

// 在另一个文件中定义被引入的模块
mod sub_module {
    pub fn sub_function() {
        // 函数体
    }
}
```

## Vector

在 rust 中，`Vec` (Vector) 是一种动态数组类型，它运行在运行时动态地增加或减少元素。

**创建 Vector**

可以使用 `vec![]` 宏或者 `Vec::new()` 的方式来创建一个新的 Vector。使用 `vec![]` 宏的方式必须要有初始值。

```rs
let v1: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];
```

**添加元素**

使用 `push` 方法将新元素添加到 Vector 的末尾。

```rs
let mut v1: Vec<i32> = Vec::new();
v1.push(1);
v1.push(2);
```

**修改元素**

通过索引直接修改 Vector 中的元素。

```rs
let mut v1 = vec![1, 2, 3];

v1[0] = 4;
let v = v1[0];

println!("{}", v); // 4
```

**访问元素**

访问 Vector 中的元素有两种方式：索引语法和 `get` 方法。索引语法访问元素越界时，会使程序奔溃。使用 `get` 方法访问越界元素时，会返回 `None`。

```rs
let mut v1 = vec![1, 2, 3];
let n = v1[0];
// let n2 = v1[3]; // index out of bounds: the len is 3 but the index is 3

let n3 = v1.get(0);
let n4 = v1.get(3);

println!("{:?}", n4); // None

if (n4 == None) {
    println!("Vector 访问越界");
}
```

**删除元素**

使用 `remove` 方法删除 Vector 中的元素。

```rs
let mut v1 = vec![1, 2, 3];

v1.remove(1); // 删除索引为 1 的元素

println!("{:?}", v1); // [1, 3]
```

**遍历**

使用 `for` 循环遍历 Vector 中的元素。

```rs
let mut v1 = vec![1, 2, 3];

for element in v1 {
    println!("{}", element);
}
```

**所有权**

当将 Vector 传递给函数时，它将转译所有权。如果你不希望函数获取所有权，可以使用引用 `&Vec<T>`。

```rs
fn process_vector(v: &Vec<i32>) {
    // 处理Vector，但不获取所有权
}
```

## 字符串

rust 中的字符串是 UTF-8 编码的，这意味着它们支持全球范围内的文本。

在 rust 中，有两种主要的字符串类型：`String` 和 `&str`。

### 字符串切片 `&str`

字符串切片是一个不可变的引用，它指向存储在其他地方的字符串数据。

**创建**

通过对已有字符串去引用来创建字符串切片。

```rs
let greeting: &str = "Hello, world!";
```

### 动态字符串 `String`

`String` 是一个堆分配的可变字符串类型，它运行在运行时动态改变其内容。

**创建**

通过 `String::from` 方法或者使用 `to_string` 方法从字符串切片创建。

```rs
// 创建一个空的 String
let mut s = String::new();

let s1 = String::from("Hello, World!");
println!("{}", s1); // Hello, World!

let data = "Hello, World!";

let s2 = data.to_string();
println!("{}", s2); // Hello, World!
```

**更新**

使用 `push_str` 方法向 `String` 附加字符串切片，使用 `push` 方法将单独的字符附加到 `String` 中。

```rs
let mut s1 = String::from("Hello");

s1.push_str(", World");
println!("{}", s1); // Hello, World

s1.push('!');
println!("{}", s1); // Hello, World!
```

使用 `+` 运算符或 `format!` 宏来拼接 `String`。

```rs
let s1 = String::from("Hello");
let s2 = String::from(", World");
let s3 = String::from("!");

let s4 = s1 + &s2 + &s3;
println!("{}", s4); // Hello, World!
```

使用 `+` 运算符来拼接 `String` 时，第一个 `+` 运算符前的字符串将会失去所有权，不再可以访问，`+` 运算符后面都是字符串的**引用**。

```rs
let s1 = String::from("Hello");
let s2 = String::from(", World");
let s3 = String::from("!");

let s4 = format!("{}{}{}", s1, s2, s3);
println!("{}", s4); // Hello, World

println!("s1: {}, s2: {}, s3: {}", s1, s2, s3); // s1: Hello, s2: , World, s3: !
```

使用 `format!` 宏来拼接 `String`，被拼接的字符串的所有权都在，都可以再访问。

**索引**

不可以使用索引语法来访问 `String`。

```rs
let s1 = String::from("Hello");
// let h = s1[0]; // the type `String` cannot be indexed by `{integer}
```

**遍历**

使用 `chars` 方法来迭代字符串的字符。

```rs
let s1 = String::from("Hello");

for c in s1.chars() {
    println!("{}", c);
}
```

## 哈希 map

`HaspMap<K, V>` 类型存储了一个键类型 `K` 对应一个值类型 `V` 的映射。

**创建**

使用 `new` 创建一个空的 `HashMap`，并使用 `insert` 增加元素。

```rs
// 导入 HashMap 模块
use std::collections::HashMap;

// 创建一个空的 HashMap
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}
```

使用元组的 `collect` 方法。

```rs
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}
```

**访问**

使用 `get` 方法并提供对应的键来从 `HashMap` 中获取值。

```rs
use std::collections::HashMap;

// 创建一个空的 HashMap
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);

println!("{:?}", score); // Some(10)
```

`get` 返回的是 `Option<V>`，结果被装进 `Some`。如果某个键在 `HashMap` 中没有对应的值，则 `get` 会返回 `None`。

**更新**

覆盖旧的值。

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 50);

println!("{:?}", scores); // {"Blue": 50}
```

只在键没有值时插入。

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Blue")).or_insert(50); // Blue 键有值，忽略
scores.entry(String::from("Yellow")).or_insert(50); // Yellow 键没有值，插入

println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}
```

**删除**

使用 `remove` 方法并提供对应的键来从 `HashMap` 中删除元素。

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

scores.remove("Blue");

println!("{:?}", scores); // {"Yellow": 50}
```

**遍历**

使用 `iter` 方法可以迭代哈希表中的键值对。

```rs
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in scores.iter() {
    println!("key: {}, value: {}", key, value); // key: Blue, value: 10 // key: Yellow, value: 50
}
```

**所有权**

对于像 `i32` 这样实现了 `Copy` trait 的类型，其值可以直接拷贝进 `HashMap`。对于像 `String` 这样拥有所有权的值，其值将被移动，`HashMap` 会成为这些值的拥有者。

```rs
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name 和 field_value 不再有效

// println!("{}, {}", field_name, field_value); // move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
```

## 错误处理

在 rust 中，错误处理是通过 `Panic` 和 `Result` 来完成的。这两者都用于处理程序在运行时可能存在的问题。

### Panic

`Panic` 是一种非常严重的错误，表示程序遇到了无法处理的情况，通常是由于编程错误导致的。当程序遇到 `Panic` 时，它会立即终止执行并打印错误信息。

**使用场景：**

- 不可恢复的错误，比如数组越界、空指针引用等。
- 在开发和调试阶段用于定位问题，但在生产代码中应该尽量避免。

### Result

`Result` 是一种用于处理可能出现错误的枚举类型。它有两个可能得值：`Ok` 和 `Err`。`Ok` 表示操作成功，`Err` 表示操作失败，并携带一个描述错误的值。

```rs
fn main() {
  fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(x / y)
    }
  }

  match divide(10, 2) {
      Ok(result) => println!("Result: {}", result),
      Err(error) => println!("Error: {}", error),
  }
}
```

上述代码中，`divide` 函数返回一个 `Result`，在 `main` 函数中使用 `match` 表达式处理可能得错误情况。

**使用 `?` 运算符简化错误传递**

在函数返回类型为 `Result` 的情况下，可以使用 `?` 运算符来简化错误传递。如果 `Result` 是 `Ok`，则将成功的值取出来；如果是 `Err`，则将错误传递给调用者。

```rs
fn main() -> Result<(), &'static str> {
  fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(x / y)
    }
  }

  let result = divide(10, 2)?;
  println!("Result: {}", result);
  Ok(())
}
```

**链式调用和错误不同类型的错误**

```rs
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
```

在上述例子中，`read_file_content` 函数尝试打开文件并读取内容，如果其中任何一个步骤失败，将立即返回相应的错误。在 `main` 函数中，我们使用 `match` 表达式来处理 `read_file_content` 的返回值，如果是 `Ok`，则打印文件内容，如果是 `Err`，则打印错误信息。

**使用场景：**

- 当函数可能返回错误时，使用 Result 来传递错误信息，而不是通过 `Panic` 终止程序。
- 允许调用方选择如何处理错误，可以通过模式匹配或者 `Result` 的方法来获取值或者处理错误。

## 泛型

使用使用泛型可以为函数签名或结构体等项创建定义，这样它们就可以用于多种不同的具体数据类型。泛型的核心思想是参数化类型，这使得你可以编写与具体类型无关的代码。

**泛型函数**

```rs
fn main() {
    fn compare<T>(a: T, b: T) -> bool
    where
        T: PartialOrd,
    {
        a < b
    }

    println!("{}", compare(1, 2));
    println!("{}", compare('a', 'b'));
}
```

上述代码中，`compare` 就是一个泛型函数，`<T>` 表示这个函数是泛型的，`T` 是一个占位符，表示任意类型。`PartialOrd` 是一个 `trait`，表示类型可以进行部分比较。

**泛型结构体**

```rs
fn main() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

上述代码中，`Point<T>` 是一个泛型结构体，`<T>` 表示这个结构体是泛型的，`T` 是一个占位符，表示任意类型。

**泛型枚举**

```rs
fn main() {
  enum Option<T> {
      Some(T),
      None,
  }
}
```

上述代码中，`Option<T>` 是一个泛型枚举，`<T>` 表示这个枚举是泛型的，`T` 是一个占位符，表示任意类型。

## Trait

在 rust 中，`trait` 是一种定义共享行为的机制，类似于其他编程语言中的接口或抽象类。`trait` 允许你在不同类型之间共享方法签名，以便它们可以实现相同的功能。这提供了一种在不同类型之间实现共享行为的方式，而无需继承。

**基本语法**

```rs
trait Shape {
    // 方法签名
    fn area(&self) -> f64;
}
```

**实现 trait**

要使类型实现一个 `trait`，可以在类型上使用 `impl` 块，并提供 `trait` 中定义的方法的具体实现。

```rs
trait Shape {
      fn area(&self) -> f64;
  }

struct Rectangle {
  width: f64,
  height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

**默认方法**

`tarit` 可以包含默认实现的方法，实现这个 `trait` 的类型可以选择覆盖这些默认实现。

```rs
fn main() {
    trait Shape {
        fn area(&self) -> f64;
        fn my_method_with_default(&self) {
          println!("Default implementation");
        }
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    let rect = Rectangle { width: 2.0, height: 3.0 };
    // 调用默认方法
    rect.my_method_with_default(); // 输出 Default implementation

    struct Circle {
        radius: f64
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        // 覆盖默认实现
        fn my_method_with_default(&self) {
            println!("radius: {}", self.radius);
        }
    }

    let circle = Circle { radius: 2.0 };
    circle.my_method_with_default(); // 输出 radius: 2
}
```

**Trait 作为参数**

```rs
fn main() {
    trait Shape {
        fn area(&self) -> f64;
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    let rect = Rectangle { width: 2.0, height: 3.0 };

    struct Circle {
        radius: f64
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    let circle = Circle { radius: 2.0 };

    fn get_area(shape: impl Shape) -> f64 {
        shape.area()
    }

    let rect_area = get_area(rect);
    let circle_area = get_area(circle);
    println!("{:?}", rect_area); // 6.0
    println!("{:?}", circle_area); // 12.566370614359172
}
```

上述代码中，`shape` 参数指定了 `impl` 关键字和 `trait` 名称，该参数支持任何实现了指定 `trait` 的类型。在 `get_area` 函数体中，可以调用任何来自 `Shape` trait 的方法，比如 `area` 。

**Trait Bound 语法**

`trait bound` 与泛型参数声明在一起，位于尖括号中的冒号后面。`impl Trait` 很方便，适用于短小的例子。`trait bound` 则适用于更复杂的场景

```rs
fn get_area(shape1: impl Shape, shape2: impl Shape) -> f64 {
    shape1.area() + shape2.area()
}

fn get_area2<T: Shape>(shape1: T, shape2: T) -> f64 {
    shape1.area() + shape2.area()
}
```

**通过 `+` 指定多个 `trait bound`**

```rs
pub fn notify(item: impl Summary + Display) {}

pub fn notify2<T: Summary + Display>(item: T) {}
```

**通过 `where` 简化 `trait bound`**

```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

## 生命周期

1. **基本概念：**

- **生命周期参数：** 生命周期参数是标识引用存在时间的标记。它们通常用单引号表示，如 `'a`, `'b` 等。生命周期参数用于描述引用的作用域，即引用存在的时间范围。
- **生命周期注解：** 生命周期注解是用来告诉编译器引用之间的关系，帮助编译器理解引用的有效性。生命周期注解的语法是在引用类型后面使用生命周期参数。
- **生命周期省略规则：** 编译器在某些情况下可以推断生命周期，因此代码中并非总是需要显示注解生命周期。这称为生命周期省略规则。

2. **函数中的生命周期：**

在函数中，生命周期通常与函数参数和返回值有关。

```rs
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

上述代码中，`'a` 生命周期参数表示输入字符串引用 `s1` 和 `s2` 有相同的生命周期。这意味着返回的引用的生命周期也与输入引用的生命周期相同。

3. **结构体中的生命周期：**

生命周期也可以在结构体中使用，以确保结构体的字段引用具有相同或不同的生命周期。

```rs
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let y = &5; // 这个引用的生命周期要长于Foo实例的生命周期
    let f = Foo { x: y };
    println!("{}", f.x);
}
```

上述代码中，结构体 `Foo` 中的字段 `x` 有生命周期 `'a`，而引用 `y` 的生命周期要长于 `Foo` 实例的生命周期。

4. **静态生命周期：**

`'static` 生命周期是一个特殊的生命周期，表示整个程序的执行时间。所有的字符串字面量都拥有 `'static` 生命周期。我们也可以像下面这样标注出来：

```rs
fn main() {
  let s: &'static str = "I have a static lifetime.";
}
```

```rs
fn foo(s: &'static str) {
    // 函数体
}

fn main() {
    let s: &'static str = "Hello, World!";
    foo(s);
}
```

在上述代码中，`'static` 生命周期确保字符串字面量在整个程序执行期间都是有效的。

5. **生命周期限制条件：**

有时候，为了确保引用的有效性，我们需要使用生命周期来约束泛型参数。

```rs
fn longest_with_an_announcement<'a, T>(s1: &'a str, s2: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement: {}", ann);
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```

上述代码中，`'a` 生命周期参数限制了泛型参数 `T` 的生命周期，确保它与输入引用的生命周期相同。

**总而言之，生命周期是 rust 中保障引用有效性的关键概念，它允许我们在编译时检查引用的正确性，从而避免悬垂引用和数据竞争。**

## 闭包

在 rust 中，**闭包**是可以保存进变量或作为参数传递给其他函数的**匿名函数**。

可以在一个地方创建闭包，然后再不同的上下文中执行闭包运算。

不同于函数，闭包允许捕获调用者作用域中的值。

```rs
fn main() {
    let add = |x, y| x + y;

    let result = add(5, 3);
    println!("{}", result);
}
```

上述代码中，`add` 就是一个闭包。闭包定义是 `add` 赋值的 `=` 之后的部分，闭包定义是以一对竖线 `|` 开始，在竖线中间指定闭包的参数。`|x, y|` 就是这个闭包的参数。
参数之后是存放闭包的大括号，如果标题只有一行，则大括号可以省略。闭包体最后一行的返回值作为调用闭包时的返回值。

**闭包类型推断和标注**

闭包不要求像 `fn` 函数那样在参数和返回值上注明类型，但是也可以添加类型标注。

```rs
fn  add_v1   (x: u32, y: u32) -> u32 { x + y };
let add_v2 = |x: u32, y: u32| -> u32 { x + y };
let add_v3 = |x, y|                  { x + y };
let add_v4 = |x, y|                    x + y  ;
```

上述代码中，展示了闭包语法如何类似于函数语法。

**不能调用一个闭包被推断为两个不同的类型**

```rs
fn main() {
    let add = |x, y| x + y;

    let result = add(5, 3);

    let s1 = String::from("Hello");
    let s2 = "World";
    let result2 = add(s1, s2);
}
```

上述代码中，尝试调用 `add` 闭包推断为两种不同的类型，这是会导致编译出错的。

```rs
use std::thread;
use std::time::Duration;

// 根据强度计算健身计划
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_num: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 6 {
        println!("先做俯卧撑{}组", expensive_result);
        println!("再做仰卧起坐{}组", expensive_result);
    } else {
        if random_num == 3 {
            println!("周三休息")
        } else {
            println!("先练力量{}组", expensive_result);
            println!("再跑步{}分钟", expensive_result * 10);
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
```

上面代码中，是一个制定健身计划的例子，先根据用户的运动强度，强度值小于 6 做俯卧撑和仰卧起坐，反之，如果是周三则消息，不是则做力量训练和跑步。
`simulated_expensive_calculation` 这个方法是一个耗时计算，在所有情况下都需要执行它，哪怕是那个完全不需要这一结果的第一个 `else` 的 `if` 中，下面我们使用闭包来优化。

```rs
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_num: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 6 {
        println!("先做俯卧撑{}组", expensive_closure(intensity));
        println!("再做仰卧起坐{}组", expensive_closure(intensity));
    } else {
        if random_num == 3 {
            println!("周三休息")
        } else {
            println!("先练力量{}组", expensive_closure(intensity));
            println!("再跑步{}分钟", expensive_closure(intensity) * 10);
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
```

上面代码中，我们将 `simulated_expensive_calculation` 改成了闭包，当 `simulated_random_number` 的值改为 3 时，即满足进入第一个 `else` 的 `if` 分支中时，不会去进行耗时运算。

**使用带有泛型和 Fn trait 的闭包**

在 rust 中，闭包有三种不同的 trait，分别是 `Fn`、`FnMut` 和 `FnOnce`。这些 trait 反映了闭包对其环境中的变量的不同访问方式。

1. **Fn trait:**

闭包可以以不可变引用的方式访问其环境中的变量。

```rs
let add = |x, y| x + y;
let result = add(3, 5);
println!("Result: {}", result);
```

2. **FnMut trait:**

闭包可以以可变引用的方式访问其环境中的变量。

```rs
let mut counter = 0;
let mut increment = || {
    counter += 1;
};
increment();
println!("Counter: {}", counter);
```

3. **FnOnce trait:**

闭包会消耗其环境中的变量，使其不再可用。

```rs
let owned_value = String::from("Hello");
let consume_closure = || {
    println!("Value: {}", owned_value);
    // owned_value 的所有权被移动到闭包中，无法再次使用
};
consume_closure();
// 下面的代码将无法编译，因为 owned_value 已经在闭包中被消耗了
// println!("Value: {}", owned_value);
```

上述制定健身计算的例子使用闭包优化后，还有一个问题是，在同一个 `if` 和 `else` 的分支中，闭包被执行了两次，尽管闭包两次返回的结果一致。下面我们来优化它：

```rs
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_num: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 6 {
        println!("先做俯卧撑{}组", expensive_result.value(intensity));
        println!("再做仰卧起坐{}组", expensive_result.value(intensity));
    } else {
        if random_num == 3 {
            println!("周三休息")
        } else {
            println!("先练力量{}组", expensive_result.value(intensity));
            println!("再跑步{}分钟", expensive_result.value(intensity) * 10);
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 5;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
```

上述代码中，使用 `Cacher` 的 `calculation` 字段存储闭包，使用 `value` 字段存储闭包的执行结果。当需要闭包的执行结果时，它调用 `value` 方法，该方法会判断 `self.value` 是否已经有了一个 `Some` 的结果值，如果有直接返回，而不会再次执行闭包。所以耗时操作不会多次执行。

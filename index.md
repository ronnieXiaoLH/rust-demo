## rust 安装

windows 系统在 [rust 官网](https://www.rust-lang.org/tools/install) 下载 `RUSTUP-INIT.EXE` 执行即可。

## 设置国内的 rust 镜像源

在你的 `.cargo` 目录下面新建 `config` 文件，并给这个文件添加如下内容：

```conf
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'ustc' # 如：tuna、sjtu、ustc，或者 rustcc

# 注：以下源配置一个即可，无需全部

# 中国科学技术大学
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
# >>> 或者 <<<
# registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
# [source.sjtu]
#registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/"

# 清华大学
# [source.tuna]
#registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
# [source.rustcc]
#registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
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

## 迭代器

在 rust 中，迭代器是一种非常强大和灵活的抽象，用于处理集合中的元素。迭代器允许你使用一种统一的方式遍历集合。

**创建迭代器**

可以使用调用集合类型的 `iter` 方法来创建一个迭代器。例如，数组、Vector 或切片，你可以这样使用：

```rs
let my_vector = vec![1, 2, 3];
let my_iterator = my_vector.iter();
```

**迭代器的方法**

一旦有了迭代器，可以使用各种迭代器的方法来处理元素。常见的方法有：

```rs
fn main() {
    let my_vector = vec![1, 2, 3];

    let squared_values: Vec<i32> = my_vector.iter().map(|x| x * x).collect();
    println!("{:?}", squared_values); // [1, 4, 9]

    let even_values: Vec<_> = my_vector.iter().filter(|&x| x % 2 == 0).collect();
    println!("{:?}", even_values); // [2]

    let sum = my_vector.iter().fold(0, |acc, &x| acc + &x);
    println!("{}", sum); // 6

    my_vector.iter().for_each(|&x| println!("{}", x));
}
```

**消费迭代器**

迭代器分为两种：可消费和不可消费。对迭代器调用可消费方法后，它就不能再被使用了。可消费方法有 `collect`、`sum`、`max`、`min`、`count` 等。

```rs
fn main() {
    let my_vector = vec![1, 2, 3];

    let squared_values: Vec<i32> = my_vector.iter().map(|x| x * x).collect();
    println!("{:?}", squared_values); // [1, 4, 9]

    let sum: i32 = my_vector.iter().sum();
    println!("{}", sum); // 6

    let max = my_vector.iter().max();
    print!("{:?}", max); // Some(3)

    let min = my_vector.iter().min();
    println!("{:?}", min); // Some(1)

    let count = my_vector.iter().count();
    println!("{}", count); // 3
}
```

**自定义迭代器**

你也可以自定义迭代器，实现 `Iterator` trait。这需要实现 `next` 方法，该方法在每次迭代时返回下一个元素或者 `None` 表示迭代结束。

```rs
fn main() {
    struct Counter {
        current: usize,
        max: usize,
    }

    impl Iterator for Counter {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let result = Some(self.current);
                self.current += 1;
                result
            } else {
                None
            }
        }
    }

    let counter = Counter { current: 0, max: 5 };
    let values: Vec<_> = counter.collect();
    println!("{:?}", values) // [0, 1, 2, 3, 4]
}
```

## Cargo 和 Cargo.io

在 rust 中，`cargo` 是一个非常重要的工具，用于管理项目的构建、依赖和发布。

`cargo` 提供了一种简化的方式来构建、测试和运行 rust 项目，同时还能够管理项目的依赖关系和发布到 `Cargo` 库(`cargo.io`)。

### Cargo

**项目管理：**

- **`cargo new：`** 创建一个新的 rust 项目
- **`cargo build：`** 构建项目，编译成可执行文件
- **`cargo run：`** 构建并运行项目
- **`caargo test：`** 运行项目中的测试
- **`cargo doc：`** 生成项目的文档

**依赖管理：**

- **`cargo update：`** 更新项目的依赖
- **`cargo upgrate：`** 升级依赖到最新的版本
- **`cargo.toml：`** 项目的配置文件，包含依赖和其他配置信息

**发布：**

- **`cargo publish：`** 将库发布到 `cargo.io`
- **`cargo package：`** 逛街一个用于发布的压缩包
- **`cargo.lock：`** 记录项目依赖的确切版本，以确保构建时可重复的

**其他命令：**

- **`cargo check：`** 检查代码但不生成可执行文件
- **`cargo clean：`** 清理项目构建产生的文件

**cargo.toml 中的发布配置**

当项目中的 `cargo.toml` 中没有任何的 `[profile.*]` 部分的时候，`cargo` 会对每一个配置都采用默认设置。`[profile.*]` 对应的部分中的配置覆盖默认的设置。例如：

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 1
```

上述代码中，`dev` 和 `release` 都设置了 `opt-level` 的值。`opt-level` 设置控制 rust 对代码进行何种程度的优化，这个配置的值是 0 到 3，值越大编译所需时间越长，代码性能越好。

**cargo 工作空间**

`cargo` 的工作空间是一种组织包含多个相关项目的结构。通过工作空间，你可以方便地管理多个项目，共享依赖关系，并在它们之间共享构建配置。

一个典型的 `cargo` 工作空间包括一个根目录，其中包含一个 `cargo.toml`文件，以及一个或多个子目录 ，每个子目录都是一个独立的 `rust` 项目。

1. **根目录中的 `cargo.toml` 文件：**

在工作空间的根目录中，需要有一个 `cargo.toml` 文件，其中包含有关工作空间的元信息和配置。该文件通常包括 `[workspace]` 部分，以指定这是一个工作空间。

```toml
# cargo.html
[workspace]
members = [
    "crate1",
    "crate2",
    "crate3"
]
```

2. **子项目目录：**

工作空间中的每个子项目都是一个独立的 rust 项目，有自己的 `cargo.toml` 文件。

每个子项目目录都是工作空间的成员，通过 `members` 字段在根目录的 `cargo.toml` 中列出。

3. **共享依赖：**

工作空间中的所有子项目可以共享依赖关系。如果多个项目都需要相同的依赖，只需要在根目录的 `cargo.toml` 中指定一次即可。

4. **构建和测试：**

在工作空间的根目录中执行 `cargo build`、`cargo test` 或 `cargo run` 将同时处理所有成员项目。

如果只想处理单个项目，可以使用 `p` 或 `--package` 选项指定需要处理的子项目。

### Cargo.io

- rust 的包管理系统，类似于其他语言的包管理工具（如 npm）
- 使用 `cargo install` 命令可以从 `cargo.io` 安装依赖

## 智能指针

### Box<T>

在 rust 中，`Box<T>` 是一个用于在堆上分配内存并存储类型 `T` 的智能指针。它提供了对在堆上分配的内存进行所有权管理的能力。

`Box<T>` 是一种指针类型，允许你在编译时确保在运行时的一些内存安全性和所有权规则。

1. **堆分配：** `Box<T>` 主要用于在堆上分配内存。`Box<T>` 允许你在堆上动态分配一块内存，并将 `T` 类型的值放在内存中。

2. **所有权管理：** `Box<T>` 提供了对内存块的独占所有权。这意味着在任何时刻，只有一个 `Box<T>` 可以拥有对堆上内存的访问权。当 `Box<T>` 被移动（赋值给另一个 `Box<T>` 或传递给函数等），所有权也被转移。

3. **解引用：** 你可以使用 `*` 运算符对 `Box<T>` 进行解引用，以获得存储在堆上的 `T` 类型的值。

4. **自动释放：** 当 `Box<T>` 超出作用域时，rust 会自动调用 `T` 类型的 `drop` 方法来释放堆上的内存。这保证了内存的正确释放，避免了常见的内存泄漏问题。

5. **使用场景：** `Box<T>` 在需要动态分配内存，或者需要确保数据的所有权关系的场景下非常有用。它通常用于构建递归数据结构、实现 `trait` 对象、或者在需要在不同函数之间传递所有权时。

```rs
fn main() {
    // 在堆上分配一个整数
    let box1: Box<i32> = Box::new(1);
    // 解引用获取堆上的值
    let value1 = *box1;
    println!("{}", value1); // 1

    let box2: Box<String> = Box::new(String::from("Hello World"));
    let value2 = *box2;
    println!("{}", value2); // Hello World
}
```

### Deref trait

在 rust 中，`Deref` trait 是一个非常重要的 trait，用于实现解引用操作。

通过实现 `Deref` trait，你可以重载解引用操作符 `*`，使得你的类型在使用解引用操作符时能够表现得像指针一样。

`Deref` trait 的定义如下：

```rs
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```

**type Target** 是一个关联类型，它指定了 `Deref` trait 实现目标类型。这个类型是解引用操作返回的类型。

**deref 方法** 用于返回被解引用的值的引用。当你使用 `*` 操作符堆实现了 `Deref` trait 的类型进行解引用时，它会被自动调用。

```rs
use std::ops::Deref;

struct MyBox<T>(T);

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let my_box = MyBox(1);
    // 使用 * 操作符进行解引用
    let value = *my_box;
    println!("{}", value);
}
```

上述代码中，`MyBox` 结构体实现了 `Deref` trait，所以它的实例 `my_box` 可以被解引用。

### Drop trait

在 rust 中，`Drop` trait 是一个用于执行资源清理操作的特殊 trait。它定义了一个单一的方法 `drop`，该方法在值即将离开作用域时被自动调用。

`Drop` trait 的定义如下：

```rs
pub trait Drop {
    fn drop(&mut self);
}
```

任何类型都可以实现 `Drop` trait，只需要实现 `drop` 方法。当一个实现了 `Drop` trait 的类型的实例离家作用域时，会自动调用其 `drop` 方法。

```rs
struct CustomType {
    data: String
}

impl Drop for CustomType {
    fn drop(&mut self) {
        println!("Dropping CustomType with data {}!", self.data);
    }
}

fn main() {
    let ct1 = CustomType { data: String::from("first") };
    let ct2 = CustomType { data: String::from("second") };
    println!("CustomType created.")
}
```

上述代码中，我们自定义了一个 `CustomType` 的结构体，并且它实现了 `Drop` trait。代码运行后会输出：

```
CustomType created.
Dropping CustomType with data second!
Dropping CustomType with data first!
```

变量丢弃的顺序和创建时的顺序相反，所以 `ct2` 在 `ct1` 之前被丢弃。

**使用 std::mem::drop 提前丢弃值**

当我们希望在作用域结束之前就强制释放变量的话，可以使用标准库提供的 `std::mem::drop`。

```rs
let ct1 = CustomType { data: String::from("first") };
std::mem::drop(ct1);
let ct2 = CustomType { data: String::from("second") };
println!("CustomType created.")
```

上述代码执行后将输出：

```
Dropping CustomType with data first!
CustomType created.
Dropping CustomType with data second!
```

### Rc<T>

在 rust 中，`Rc<T>` 是一个引用计数智能指针（Reference Counted smart poiter）的类型。它在内部使用引用计数来追踪指向堆上数据的引用数量。

`Rc` 用于多个所有者之间共享数据，允许多个部分共享对同一块内存的访问，而无需担心所有权的问题。

1. **创建和引用计数：**

使用 `Rc<T>` 时，首先需要将数据包装在 `Rc` 中。

```rs
use std::rc::Rc;

fn main() {
    let a = Rc::new(1);
    println!("count after creating a = {}", Rc::strong_count(&a));
}
```

上述代码中，创建了一个包含整数 1 的 `Rc` 的实例。此时，引用计数为 1，因为这是第一个拥有者。

2. **克隆和引用计数递增**

当你想要共享数据的时候，可以通过调用 `clone` 方法来创建指向同一块内存的新引用，同时增加引用计数。

```rs
use std::rc::Rc;

fn main() {
    let a = Rc::new(1);
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1

    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
}
```

现在，`a` 和 `b` 都指向相同的数据，并且引用计数增加为 2。

3. **减少引用次数**

当不再需要某个引用时，可以将其从作用域中移除，从而减小引用次数。

```rs
use std::rc::Rc;

fn main() {
    let a = Rc::new(1);
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1

    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}
```

上述代码中，当 `c` 创建的时候，引用计数增加为 3，当 `c` 离开作用域后，引用计数减小为 2。

### RefCell<T>

**内部可变性**是 rust 中的一个设计模式，它允许你即使在有不可变引用时，也可以改变数据，这是借用规则所不允许的。

`RefCell<T>` 是 rust 标准库中的一个类型，用于提供运行时借用检查而不是在编译时进行检查的内部可变性。

在 rust 中，有一条规则，即同一时刻只能有一个可变引用或多个不可变引用，确保在运行时不会出现数据竞争。`RefCell<T>` 允许你绕过这个规则，代价是在运行时进行借用检查。

`RefCell<T>` 的主要特点是，它允许在不可变引用的情况下修改包含在其中的值。

在运行时，`RefCell<T>` 会检查是否存在多个可变引用，如果存在，会导致 `panic`。通过 `borrow` 和 `borrow_mut` 方法分别获取不可变引用和可变引用。

```rs
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);

    // 获取不可变引用
    let borrowed = data.borrow();
    let borrowed2 = data.borrow();
    println!("{}, {}", borrowed, borrowed2); // 1, 1
}
```

上述代码是可以正常运行的，因为可以同时存在多个不可变引用。

```rs
use std::cell::RefCell;

fn main() {
    // 获取可变引用
    let mut borrow_mut1 = data.borrow_mut();
    let mut borrow_mut2 = data.borrow_mut();
}
```

上述代码会出现 `already borrowed: BorrowMutError` 的错误，因为不可以同时存在多个可变引用。

```rs
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);

    // 获取不可变引用
    let borrowed = data.borrow();

    // 获取可变引用
    let mut borrow_mut = data.borrow_mut();
}
```

上述代码也是错误的，因为同时出现了不可变引用和可变引用。

```rs
use std::cell::RefCell;

// 定义一个结构体，包含一个 RefCell 用于内部可变性
struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new(initial: i32) -> Counter {
        Counter {
            value: RefCell::new(initial),
        }
    }

    // 获取当前值的不可变引用
    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }

    // 增加值，在不可变引用的情况下修改数据
    fn increment(&self) {
        let mut val = self.value.borrow_mut();
        *val += 1;
    }
}

fn main() {
    let counter = Counter::new(1);

    // 获取不可变引用并输出当前值
    println!("Initial value: {}", counter.get_value()); // 1

    // 在不可变引用的情况下调用增加值的方法
    counter.increment();

    // 再次获取不可变引用并输出修改后的值
    println!("Initial value: {}", counter.get_value()); // 2
}
```

上述代码中，演示了在不可变引用的同时修改数据的情况。在 `get_value` 方法中以不可变引用的方式获取值，并在 `increment` 方法中以可变引用的方式修改值。

**选择 `Box<T>`、`Rc<T>` 或 `RefCell<T>` 的理由：**

- `Rc<T>` 允许相同的数据有多个所有者；`Box<T>` 和 `RefCell<T>` 为单一所有者。
- `Box<T>` 允许在编译时执行不可变或可变借用检查；`Rc<T>` 仅运行在编译时执行不可变借用检查；`RefCell<T>` 允许在运行时执行不可变或可变借用检查。
- 因为 `RefCell<T>` 允许在运行时执行不可变或可变借用检查，所以我们可以在即便 `RefCell<T>` 自身不可变的情况下修改其内部的值。

## 并发

在 rust 中，提供了一套强大而安全的多线程编程工具，允许你创建和管理线程，以及线程间的通信。多线程编程时通过 `std::thread` 模块来实现的。

1. **创建线程**

使用 `std::thread::spawn` 函数可以创建一个新的线程，这个函数接受一个闭包（或函数）作为参数，表示该线程的主题逻辑。

```rs
use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}
```

上述代码中，创建了一个新的线程，但是新的线程的逻辑并不能保证全部执行完。

可以通过将 `thread::spawn` 的返回值存在变量中来修复新建线程部分没有执行或完全没有时间执行的问题。`thread::spawn` 的返回值类型是 `JoinHandle`，它是一个拥有所有权的值，当其调用 `join` 方法时，它会等待其线程结束。

```rs
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join().unwrap();
}
```

2. **线程与 move 闭包**

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

上述代码会出现 `closure may outlive the current function, but it borrows `v`, which is owned by the current function` 的错误。原因是 `println!` 需要 `v` 的引用，闭包尝试借用 `v`，但是 rust 并不知道这个新建线程会执行多久，所以无法知晓 `v` 的引用是否一直有效。

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

上述代码中，通过在闭包之前增加 `move` 关键字，强制闭包获取其使用值的所有权。

3. 线程间消息传递

在 rust 中，支持通过通道进行线程间的消息传递。

```rs
use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

上述代码中，使用 `mpsc::channel` 方法创建一个新的通道。`mpsc` 是**多个生产者，单个消费者（multiple producer,single comsumer）**的缩写。`mpsc::channel` 方法返回一个元素，第一个元素是发送端，第二个元素是接收端。

通道的接收端有两个有用的方法：`recv` 和 `try_recv`。`recv` 会阻塞主线程执行直到从通道中接收到值。`try_recv` 不会阻塞，相反它立刻返回一个 `Result<T, E>`：`Ok` 值包含可用的信息，`Err` 值表示此时没有任何消息。

```rs
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

上述代码中，当通道的发送端 `tx` 发送多条消息的时候，需要将 `rx` 当做一个迭代器。当通道关闭时，迭代器也将结束。

```rs
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

上述代码中，通过克隆发送者来创建了多个生产者。

4. **共享状态并发**

在 `rust` 中，提供了 `std:sync` 模块，其中包含了一些用于多线程通信的工具，如 `Mutex`、`Arc` 等。

`Mutex` （互斥锁）允许多个线程访问共享数据，但一次只有一个线程可以对数据进行修改。

```rs
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(1);

    {
        let mut num = m.lock().unwrap();
        *num = 2;
    }

    println!("{}", m.lock().unwrap()); // 2
}
```

上述代码中，演示了在单线程上下文中使用 `Mutex`。

```rs
use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap()); // 10
}
```

上述代码中，演示了使用 `Arc<T>` 包装一个 `Mutex<T>` 实现多线程之间共享所有权。

在 rust 中，`Arc<T>` （原子引用计数）是一种智能指针，用于在多线程环境中共享数据。`Arc` 提供了引用计数的功能，以便在堆上分配的数据可以被多个所有者共享，并且在所有所有者都不再需要数据时自动释放。

## 面向对象特性

rust 是一种系统级别的编程语言，它强调安全性、并发性和性能。与一些其他语言不同，rust 并不是一种传统的面向对象编程语言，它更强调所有权系统和借用检查器来管理内存安全。

然而，rust 仍然具有一些面向对象编程风格的特性。

**结构体：** 在 rust 中，你可以使用结构体定义数据结构，这与面向对象编程中的类相似。结构体还可以定义方法。

```rs
struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

**Trait：** rust 中的 trait 类似于其他语言中的接口，它定义一组方法，结构体或枚举可以实现这些方法。trait 允许在不同类型之间共享行为。

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

struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
```

## 模式语法

**\_**

在 rust 中，如果你定义了一个变量，却不在任何地方使用它，通常会给你一个警告，如果你再这个变量前面加上一个 `_`，将不会得到警告。

```rs
let x = 1;
let _y = 2;
```

上述代码中，变量 `x` 会得到一个警告 `unused variable: x`，变量 `y` 没有警告。

```rs
fn main() {
    let s1 = String::from("hello");
    let _ = s1;
    println!("{:?}", s1);

    let s2 = String::from("hello");
    let _s = s2;
    println!("{:?}", s2); // borrow of moved value: `s2`
}
```

上述代码中，将 `s1` 的值赋值给 `_`，`s1` 的所有权还是在它自己，所以赋值后 `s1` 还可以访问。将 `s2` 的值赋值给 `_s` 后，所有权也一并转移了，所以赋值后 `s2` 不可以访问。

```rs

fn main() {
    let s = Some(String::from("hello"));

    if let Some(_) = s {
        println!("found a string");
    }

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s); // borrow of partially moved value: `s`
}
```

这个例子中，同上。

**匹配守卫**

在 rust 中，匹配守卫是一种在 `match` 表达式中使用的功能，用于在模式匹配时添加额外的条件。这使得在匹配中可以使用更复杂的条件，而不仅仅是模式本身。

匹配守卫是通过 `if` 关键字引入的条件表达式来实现的。在 `match` 语句中，当模式匹配成功时，如果存在匹配守卫，将会进一步检查守卫中的条件，只有当守卫条件为真时，响应的代码块才会执行。

```rs
fn main() {
    let value: i32 = 25;

    match value {
        1 | 2 | 3 if value > 0 => {
            println!("Small positive number");
        }
        10..=50 if value % 2 == 0 => {
            println!("Even number in the range 10 to 50");
        }
        _ => {
            println!("Something else");
        }
    };
}
```

上述代码中，有两个匹配守卫：`1 | 2 | 3 if value > 0` 和 `10..=50 if value % 2 == 0`，分别匹配值为 1、2 或 3，并且同时要求值大于 0，匹配值在 10 到 50 的范围内，并且要求值为偶数。当值为 25 时，两个匹配守卫都不满足，所以输出 `Something else`。

**@绑定**

at 运算符（`@`）允许我们在创建一个存放值的变量的同时，测试其值是否匹配模式。

```rs
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
```

上述代码运行后，会打印出 `Found an id in range: 5`。通过在 `3..7` 之前指定 `id_variable @`，我们捕获了任何匹配此范围的值并同时测试其值匹配这个范围模式。

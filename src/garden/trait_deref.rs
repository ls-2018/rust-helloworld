// 一种类型转换成另一种类型,但是要在引用符号 &、点号操作符 .
// 或其他智能指针的触发下才会产生转换。

// 还有 &Vec<T> 可以自动转换为 &[T]，也是因为 Vec[T] 实现了 Deref。

// 比如标准库里最常见的 &String 可以自动转换到 &str ，就是因为 String 类型实现了
// Deref trait。 还有 &Vec 可以自动转换为 &[T]，也是因为 Vec[T] 实现了 Deref。

use std::ops::Deref;

// 定义一个结构体 C，它包含一个 i32 类型的值
struct C(i32);

// 为 C 实现 Deref trait
impl Deref for C {
    type Target = i32;

    // 指定 Deref 的目标类型

    fn deref(&self) -> &Self::Target {
        &self.0 // 返回对内部 i32 值的引用
    }
}

fn main() {
    let c = C(10);

    println!("The value is: {}", *c); // 使用 * 来解引用 c
}

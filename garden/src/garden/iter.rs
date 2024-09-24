#[test]

fn main() {
    use super::base::User;
    // use crate::garden::User; // 显式导入相对于crate根路径的语法项
    let a: Vec<u32> = vec![1, 2, 3, 4, 5];

    let mut b: Vec<u32> = vec![1, 2, 3, 4, 5];

    let mut an_iter = a.iter(); // 将Vec转换为迭代器
    while let Some(i) = an_iter.next() {
        // 调用 .next() 方法
        println!("{i}");
    }

    println!("{:?}", a);

    for item in &a {
        // 等价于 iter
        println!("{item}");
    }

    for item in &mut b {
        // 等价于 iter_mut
        println!("{item}");
    }

    for item in a {
        // 等价于 into_iter
        println!("{item}");
    }

    // 获取集合元素不可变引用的迭代器，对应方法为 iter()。
    // 获取集合元素可变引用的迭代器，对应方法为 iter_mut()。
    // 获取集合元素所有权的迭代器，对应方法为 into_iter()。//
    // 实际会复制一份这个数组
}

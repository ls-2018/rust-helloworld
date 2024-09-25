use std::collections::HashSet;
use actix_web::middleware::from_fn;
use rand::random;

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

    // 获取集合元素不可变引用的迭代器，对应方法为 iter()
    // 获取集合元素可变引用的迭代器，对应方法为 iter_mut()
    // 获取集合元素所有权的迭代器，对应方法为 into_iter()
    // 实际会复制一份这个数组

    use rand::random;
    use std::iter::from_fn;

    let x: Vec<_> = from_fn(|| { Some((random::<i32>() - random::<i32>()).abs()) }).take(3).collect();
    println!("{:?}", x);
}

use std::iter::successors;
use num_complex::Complex;

#[test]
fn asd() {
    println!("{:?}", escape_time(Complex { re: 2f64, im: 3f64 }, 5));
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));

    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let zero = Complex { re: 0.0, im: 0.0 };
    successors(Some(zero), |&z| { Some(z * z + c) })
        .take(limit)
        .enumerate()
        .find(|(_i, z)| z.norm_sqr() > 4.0)
        .map(|(i, _z)| i)
}


use std::iter::Peekable;

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
where
    I: Iterator<Item=char>,
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n
        }
        tokens.next();
    }
}
#[test]
fn p() {
    let mut chars = "226153980,1766319049".chars().peekable();
    assert_eq!(parse_number(&mut chars), 226153980);
    // 注意，`parse_number`并没有消耗这个逗号，所以我们能看到它
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 1766319049);
    assert_eq!(chars.next(), None);
}
struct Flaky(bool);

impl Iterator for Flaky {
    type Item = &'static str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("totally the last item")
        } else {
            self.0 = true; // 糟糕！
            None
        }
    }
}

#[test]
fn a1() {
    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("totally the last item"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("totally the last item"));

    let mut not_flaky = Flaky(true).fuse();
    assert_eq!(not_flaky.next(), Some("totally the last item"));
    assert_eq!(not_flaky.next(), None);
    assert_eq!(not_flaky.next(), None);

    let x = "große".chars();


    let upper_case: String = "große".chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after:     {:?}", c))
        .collect();
    assert_eq!(upper_case, "GROSSE");

    let v: Vec<i32> = (1..4).chain([20, 30, 40]).rev().collect();
    assert_eq!(v, [40, 30, 20, 3, 2, 1]);


    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);


    let message = "To: jimb\r\n\
               From: id\r\n\
               \r\n\
               Oooooh, donuts!!\r\n";

    let mut lines = message.lines();

    println!("Headers:");
    //  借出一个对迭代器的可变引用
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }
    // println!("\nBody:");
    // for body in lines {
    //     println!("{}" , body);
    // }
    let a = ['1', '2', '3', '∞'];
    // 不拥有任何资源的简单类型可以是 Copy 类型，对这些简单类型赋值会创建源的副本，而不会移动值并使源回到未初始化状态。
    let x = a.iter().cloned().next();
    let x = a.iter().copied().next();
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));

    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));
    assert_eq!(spin.next(), Some(&"South"));
    assert_eq!(spin.next(), Some(&"West"));
    assert_eq!(spin.next(), Some(&"North"));
    assert_eq!(spin.next(), Some(&"East"));


    let a = [5, 6, 7, 8, 9, 10];

    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);        // 计数
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);       // 求和
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200);   // 乘积

    // 最大值
    assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max), 10);


    let mut squares = (0..10).map(|i| i * i);

    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(0), Some(25));
    assert_eq!(squares.nth(6), None);

    let mut squares = (0..10).map(|i| i * i);
    assert_eq!(squares.next_back(), Some(81)); // 会从前面开始消耗所有迭代器的条目
    assert_eq!(squares.last(), Some(64)); // 会从前面开始消耗所有迭代器的条目


    let args = std::env::args().collect::<Vec<String>>();
    let args = std::env::args().collect::<HashSet<String>>();

    let things = ["doorknob", "mushroom", "noodle", "giraffe", "grapefruit"];

    // 惊人的事实：在这个列表里生物的名字都是以奇数序的字母开头的
    let (living, nonliving): (Vec<&str>, Vec<&str>)
        = things.iter().partition(|name| name.as_bytes()[0].lt(&101));

    assert_eq!(living, vec!["doorknob"]);
    assert_eq!(nonliving, vec!["mushroom", "noodle", "giraffe", "grapefruit"]);
}
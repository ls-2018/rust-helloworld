trait Drop {
    fn drop(&mut self);
}

struct A;

impl Drop for A {
    fn drop(&mut self) {
        println!("------");
        // 可以尝试在这里打印点东西看看什么时候调用
    }
}

#[test]

fn main() {
    let a = A;

    println!("{}", "asd")
}

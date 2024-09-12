pub fn echo(a: i32) -> i32 {
    a
}

#[cfg(test)] // 这里配置测试模块
mod tests {

    use crate::garden::vegetables::echo;

    #[test] // 具体的单元测试用例
    fn echo_t1() {
        let result = echo(1); // 调用被测试的函数或功能
        assert_eq!(result, 1); // 断言
    }
}

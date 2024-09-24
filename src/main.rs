// #![feature(trace_macros)]

fn main() {
    build_vector();

    // 我想知道这个使用assert_eq!的代码替换（展开）后会是什么样子！
    // trace_macros!(true);
    assert_eq!(10 * 10 * 10 + 9 * 9 * 9, 12 * 12 * 12 + 1 * 1 * 1);
    // trace_macros!(false);
}

// Box：指向堆中值的拥有型指针
// Box: <Attend>

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero_error() {
    1 / 0; // 应该panic！
}

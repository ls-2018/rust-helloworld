// trait FnOnce<Args> {
//     type Output;
//     fn call_once(self, args: Args) -> Self::Output;
// }
// trait FnMut<Args>: FnOnce<Args> {
//     fn call_mut(&mut self, args: Args) -> Self::Output;
// }
// trait Fn<Args>: FnMut<Args> {
//     fn call(&self, args: Args) -> Self::Output;
// }


// 获取了上下文环境变量的所有权，对应 FnOnce。            代表的闭包类型只能被调用一次
// 只获取了上下文环境变量的 &mut 引用，对应 FnMut。       代表的闭包类型可以被调用多次，并且能修改上下文环境变量的值，不过有一些副作用，在某些情况下可能会导致错误或者不可预测的行为。
// 只获取了上下文环境变量的 & 引用，对应 Fn。             代表的闭包类型可以被调用多次，但是对上下文环境变量没有副作用

#[test]
fn mainFnOnce() {
    let range = 0..10;
    let get_range_count = || range.count();
    assert_eq!(get_range_count(), 10); // ✅
    // get_range_count(); // ❌
}

#[test]
fn mainFnMut() {
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let mut min = i32::MIN;
    let ascending = nums.into_iter().filter(|&n| {
        if n <= min {
            false
        } else {
            min = n;  // 这里修改了环境变量min的值
            true
        }
    }).collect::<Vec<_>>();
    assert_eq!(vec![0, 4, 8, 10, 15, 18], ascending); // ✅
}

#[test]
fn mainFn() {
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let min = 9;
    let greater_than_9 = nums.into_iter().filter(|&n| n > min).collect::<Vec<_>>();
    assert_eq!(vec![10, 15, 18, 13], greater_than_9); // ✅
}
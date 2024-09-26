#[allow(unused_variables)]
#[allow(dead_code)]
mod _async;

use std::rc::Rc;
use std::*;
// 原子化引用计数 Arc， 复制 Arc 智能指针而不是整个 GigabyteMap。这相当于增加一次引用计数
use lazy_static::lazy_static;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::thread::spawn;

struct GigabyteMap {}

fn process_files_in_parallel(glossary: Arc<GigabyteMap>) -> io::Result<()> {
    let mut thread_handles = Vec::new();
    for worklist in 1..=8 {
        // 对.clone()的调用只会克隆Arc并增加引用计数，并不会克隆GigabyteMap
        let glossary_for_child = glossary.clone();
        thread_handles.push(spawn(move || process_files(worklist, &glossary_for_child)));
    }
    for p in thread_handles {
        p.join().unwrap();
    }

    Ok(())
}

#[allow(unused_variables)]
fn process_files(p0: i32, p1: &Arc<GigabyteMap>) {}

fn mai1n() {
    let g = GigabyteMap {};
    process_files_in_parallel(Arc::new(g)).unwrap();
    let _ = mpsc::sync_channel::<i32>(10000);
    let (sender, receiver) = mpsc::channel(); // 线程通道
    let _ = sender.send("aa").is_err();
    println!("{:?}", receiver.recv());
    println!("Hello, world!");

    let c = Condvar::new();
    c.notify_one();
    c.notify_all();
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: i32,
}

const fn mono_to_rgba(level: u8) -> Color {
    Color {
        red: level,
        green: level,
        blue: level,
        alpha: 0xFF,
    }
}

const WHITE: Color = mono_to_rgba(255);
const BLACK: Color = mono_to_rgba(000);
lazy_static! {
    // 使用 lazy_static! 会在每次访问静态数据时产生很小的性能成本
    // 通过 lazy_static! 宏定义的变量允许你使用任何喜欢的表达式进行初始化，该表达式会在第一次解引用变量时运行，并保存该值以供后续操作使用。
    static ref HOSTNAME2: Mutex<String> = Mutex::new(String::new());
}
static HOSTNAME: Mutex<String> = Mutex::new(String::new());

fn global_static_var() {
    // Rust 具有用于安全地共享变化的值的类型：Mutex、RwLock 和原子化类型。

    // 原子化全局变量仅限于简单的整数和布尔值。
    // 全局变量必须以某种方式成为线程安全的
    static PACKETS_SERVED: AtomicUsize = AtomicUsize::new(0);
    PACKETS_SERVED.fetch_add(1, Ordering::SeqCst);

    let cancel_flag = Arc::new(AtomicBool::new(false));
    cancel_flag.store(true, Ordering::SeqCst);
    let worker_cancel_flag = cancel_flag.clone();
    worker_cancel_flag.load(Ordering::SeqCst); // 内存排序
}

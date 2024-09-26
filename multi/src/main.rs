use std::*;
// 原子化引用计数 Arc， 复制 Arc 智能指针而不是整个 GigabyteMap。这相当于增加一次引用计数

use std::sync::{mpsc, Arc, Condvar};
use std::thread::spawn;


struct GigabyteMap {}

fn process_files_in_parallel(glossary: Arc<GigabyteMap>) -> io::Result<()>
{
    let mut thread_handles = Vec::new();
    for worklist in 1..=8 {
        // 对.clone()的调用只会克隆Arc并增加引用计数，并不会克隆GigabyteMap
        let glossary_for_child = glossary.clone();
        thread_handles.push(
            spawn(move || process_files(worklist, &glossary_for_child))
        );
    };
    for p in thread_handles {
        p.join().unwrap();
    }

    Ok(())
}

#[allow(unused_variables)]
fn process_files(p0: i32, p1: &Arc<GigabyteMap>) {}


fn main() {
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


    use std::sync::atomic::*;
    let atom = AtomicIsize::new(0);
    atom.fetch_add(1, Ordering::SeqCst);


    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    let cancel_flag = Arc::new(AtomicBool::new(false));
    cancel_flag.store(true,Ordering::SeqCst);
    let worker_cancel_flag = cancel_flag.clone();

    worker_cancel_flag.load(Ordering::SeqCst); // 内存排序



}

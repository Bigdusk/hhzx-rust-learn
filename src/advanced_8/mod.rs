use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

use std::hint;
#[test]
fn main() {
    //原子性操作,不会出现脏读
    let spinlock = Arc::new(AtomicU64::new(1));
    //克隆一份引用
    let spinlock_clone = Arc::clone(&spinlock);
    let thead = thread::spawn(move|| {
        //交叉执行
        spinlock_clone.store(2,Ordering::SeqCst);
    });

    //等待子线程释放锁
    while spinlock.load(Ordering::SeqCst) != 2 {
        hint::spin_loop();
    }
    thead.join().unwrap();
}
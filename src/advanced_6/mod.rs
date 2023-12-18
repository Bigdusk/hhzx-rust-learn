use std::{
    thread,
    time
};
use std::cell::RefCell;
use std::sync::Once;
//创建线程局部变量
thread_local!(static FOO: RefCell<u32> = RefCell::new(3));
//全局变量
static mut VAL: usize = 100;
//只执行因此的方法
static INIT: Once = Once::new();
#[test]
fn main() {
    //多线程编程
    //创建线程
    let thread_one = thread::spawn(move || unsafe {
        INIT.call_once(|| unsafe {
            VAL = 200;
        });
        println!("{}", VAL);
        FOO.with(|f| {
            //borrow将数据从refcell指针里面获取出来
            println!("{:?}", *f.borrow());
        })
    });
    let thread_tow = thread::spawn(move || {
        println!("子线程2")
    });
    thread_one.join().unwrap();
    thread_tow.join().unwrap();
    thread::sleep(time::Duration::from_millis(1000))
}
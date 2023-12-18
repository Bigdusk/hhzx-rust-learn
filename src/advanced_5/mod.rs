use std::cell::RefCell;
use std::sync::Arc;
use std::thread;
use std::time;
#[test]
fn main() {
    //rc与arc,前者单线程后者多线程，用于增加变量的引用次数
    let arc = Arc::new("多线程引用");
    //多线程使用数据共享，让世界再次伟大
/*    for _ in 0..=10 {
        let a = Arc::clone(&arc);
        //将变量放入线程中
        thread::spawn(move || {
            println!("{}", a)
        });
    }
    //暂停看看运行的效果
    thread::sleep(time::Duration::from_millis(10000));*/
    //增加引用次数
    let a1 = Arc::clone(&arc);
    let a2 = Arc::clone(&arc);
    let a3 = Arc::clone(&arc);
    println!("{}", a1);
    println!("{}", a2);
    println!("{}", a3);


    //RefCell实现编译期可变、不可变引用共存
    let mut cell = RefCell::new("数据");
    //实现set与get
    println!("{}", cell.get_mut())
}
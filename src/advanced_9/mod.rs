use std::sync::atomic;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

//全局变量
const NUM: usize = 100;
//静态全局变量
static mut NUMTOW: Option<&mut i32> = None;
//原子类型变量初始化，线程安全
static AA: atomic::AtomicU64 = atomic::AtomicU64::new(300);
//全局id生成器
static ID_AUTO: AtomicUsize = AtomicUsize::new(0);
//最大id值
const MAX_ID: usize = usize::MAX / 2;
//上面都是编译时初始化，下面是运行时初始化，可以在运行时用方法初始化
#[test]
fn main() {
    //全局变量
    println!("{}", NUM);
    //可变的必须使用unsafe
    //Rust 要求必须使用unsafe语句块才能访问和修改static变量，
    // 因为这种使用方式往往并不安全，其实编译器是对的，当在多线程中同时去修改时，会不可避免的遇到脏数据。
    //只有在同一线程内或者不在乎数据的准确性时，才应该使用全局静态变量。

    //原子全局变量
    println!("{:?}", AA);

    //全局id生成
    println!("{:?}", Factory::new());

    //从方法里面返回数值到全局变量
    unsafe {
        //Box::leak(Box::new(400))将变量转为可变应用&mut i32
        NUMTOW = Some(Box::leak(Box::new(400)));
    }
    unsafe { println!("{:?}", NUMTOW); }
}

#[derive(Debug)]
struct Factory {
    factory_id: usize
}

impl Factory {
    //Self代表自己的类型
    fn new() -> Self {
        Self {
            factory_id: generate_id()
        }
    }
}

fn generate_id() -> usize {
    //检查两次溢出，一直加值会导致溢出,load作用就是让变量继承规则
    let current_val = ID_AUTO.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("溢出");
    }
    ID_AUTO.fetch_add(1, Ordering::Relaxed);
    let next_id = ID_AUTO.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("溢出");
    }
    next_id
}


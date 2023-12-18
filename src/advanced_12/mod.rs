#[test]
fn main() {
    //unsafe通往地狱的大门

    let mut r = 5;
    //基于引用创建裸指针
    //可以绕过 Rust 的借用规则，可以同时拥有一个数据的可变、不可变指针，甚至还能拥有多个可变的指针
    //并不能保证指向合法的内存
    //可以是 null
    //没有实现任何自动的回收 (drop)
    //创建是安全的,而且不能直接使用变量，记得加&和&mut
    let r1 = &r as *const i32;//不可变
    let r2 = &mut r as *mut i32;//可变
    unsafe {
        //解引用是不安全的
        println!("{} {}", *r1, *r2);
        my_unsafe();
    }
}
//声明unsafe函数
unsafe fn my_unsafe() {

}
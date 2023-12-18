use std::pin::Pin;
use futures::executor::block_on;

#[test]
fn main() {
    //在异步里面使用值要用pin包裹
    let a = unsafe { Pin::new_unchecked("dasd") };
    //异步
    let async_hi = async_hi(a);
    //调用执行器
    block_on(async_hi);
    //多个方法等待

    //耻辱下播，rust快要把我干死了
}
//Create an asynchronous function
async fn async_hi(a: Pin<&str>) {
    println!("hello{}", a);
    //多个等待
    futures::join!(async_world_one(), async_world_tow());
}
async fn async_world_one() {
    println!("world one")
}
async fn async_world_tow() {
    println!("world tow")
}
#[test]
fn main() {
    //闭包与迭代器
    //闭包，当作函数的另一个简约实现形式
    let what_to_say = |say: &str| {
      println!("{}", say)
    };
    what_to_say("你好世界QvQ");
    //测试传入闭包
    one(|num| {num + 100});

    //将变量移入闭包
    let a:i32 = 100;
    let fn_a = move || {
        a
    };
    println!("{}", fn_a());

    //将闭包设为返回值
    //tow(50)


}

//将闭包设为返回值
fn tow(x: isize) -> impl  Fn(isize) -> isize{
    let num = 5;
    move |x| num + x
}

//方法传递闭包
fn one<F>(f: F)
where F: Fn(isize) -> isize,
{
    //运行传入的闭包
    println!("{}", f(32))
}
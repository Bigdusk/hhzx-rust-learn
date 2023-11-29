#[test]
fn main() {
    //所有权与借用

    //动态数组类型,可扩展与变动存在堆上面
    let mut s: String = String::from("hello");
    //追加方法
    s.push_str(", world");
    println!("{}", s);
    //变量绑定背后的数据交互
    //基础数据类型，会进行自动拷贝。数据足够简单速度是很快的
    let a = 1;
    let b = a;
    println!("a: {}, b: {}", a, b);
    //但是string不行，他是复杂数据类型
    let s2: String = String::from("hhhhh");
    let S3: String = s2;
    //对于复杂数据类型，s2的所有权会转移s3，从而报错
    //println!("s2: {}, s3: {}", s2, s3);

    //用切片&str
    let s4: &str = "我是切片";
    let s5: &str = s4;
    //答：进行了浅拷贝s4调用了引用。并不会增加存储和消耗。
    //浅拷贝只发生在栈上，因此性能很高，在日常编程中。
    println!("s3: {}, s4: {}", s4, s5);

    //深拷贝,
    let s6 = String::from("hello");
    let s7 = s6.clone();
    //因此说明 s2 确实完整的复制了 s1 的数据。
    //使用 clone 会极大的降低程序性能，需要小心使用！
    println!("s6 = {}, s7 = {}", s6, s7);
    //函数传值与返回,基础类型也可以说实现了copy的类型进入作用域后可以继续。
    //但是string的类型会直接将所有权转移进去后续无法使用变量
    test1();

    //引用与解引用
    let v = 5;
    //使用不可变引用
    //& 符号即是引用，它们允许你使用值，但是不获取所有权，如图所示：
    let v2 = &v;

    assert_eq!(5, v);
    //利用解引用获取所有权
    assert_eq!(5, *v2);

    //可变引用
    //可变引用同时只能存在一个
    // 不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用：
    let mut v4 = String::from("hi");
    let v5 = &mut v4;
    //多个不可变引用，单个可变引用
    //let v6 = &mut v4;
    println!("{}", v5);
}

fn test1() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);
    println!("{}", x);
    // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(mut some_integer: i32) { // some_integer 进入作用域
    //数据为深拷贝进来所以不影响外面数据
    some_integer += 5;
    println!("fn{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
